#![no_std]

use embassy_executor::Spawner;
use embassy_net::{Stack, tcp::TcpSocket};
use embassy_time::Duration;
use embedded_byteorder::ReadExactError;
use minecrevy_encdec::{
    AsyncDecode, AsyncReadMinecraftExt, AsyncWriteMinecraftExt, WritePacketError,
};
use minecrevy_log::{assert_eq, info};
use minecrevy_protocol::r770::{
    Handshake, NextState, StatusPing, StatusRequest, StatusResponseSimple,
};
use thiserror::Error;

const MAX_CONNECTIONS: usize = 10;

pub fn spawn_connection_tasks(spawner: Spawner, stack: Stack<'static>) {
    for id in 0..MAX_CONNECTIONS {
        spawner.spawn(connection_task(stack, id)).unwrap();
    }
}

#[embassy_executor::task(pool_size = MAX_CONNECTIONS)]
async fn connection_task(stack: Stack<'static>, id: usize) {
    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];

    loop {
        let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
        socket.set_timeout(Some(Duration::from_secs(10)));

        if let Err(e) = socket.accept(25565).await {
            info!("Socket {} failed to accept: {:?}", id, e);
            continue;
        }

        info!(
            "Socket {} connected from {:?}",
            id,
            socket.remote_endpoint()
        );

        let handshake: Handshake = match socket
            .read_packet(async move |id, mut reader| {
                assert_eq!(id, 0x00, "Handshake packet ID mismatch");
                Handshake::decode(&mut reader, ()).await
            })
            .await
        {
            Ok(handshake) => handshake,
            Err(e) => {
                info!("Socket {} failed to read handshake: {:?}", id, e);
                continue;
            }
        };

        info!("Socket {} received handshake: {:?}", id, handshake);

        if handshake.protocol_version != 770 {
            info!(
                "Socket {} received unsupported protocol version: {}",
                id, handshake.protocol_version
            );
            continue;
        }

        match handshake.next_state {
            NextState::Status => handle_status_packets(socket, id).await,
            _ => {
                info!(
                    "Socket {} received unsupported next state: {:?}",
                    id, handshake.next_state
                );
                continue;
            }
        }
    }
}

const STATUS_RESPONSE_SIMPLE: StatusResponseSimple<'static> = StatusResponseSimple(
    r#"{"version":{"name":"1.21.5","protocol":770},"players":{"max":0,"online":0},"description":"Hello, world!","enforcesSecureChat":false}"#,
);

async fn handle_status_packets(mut socket: TcpSocket<'_>, id: usize) {
    loop {
        let (mut reader, mut writer) = socket.split();

        let result = reader
            .read_packet(async move |id, mut reader| {
                match id {
                    0x00 => {
                        let Ok(request) = StatusRequest::decode(&mut reader, ()).await;
                        info!("Received status request: {:?}", request);
                        writer
                            .write_packet(0x00, STATUS_RESPONSE_SIMPLE, ())
                            .await
                            .map_err(StatusPacketError::Response)?;
                        // writer
                        //     .write_packet(0x00, STATUS_RESPONSE, Default::default())
                        //     .await
                        //     .expect("failed to write response");
                        writer.flush().await.map_err(StatusPacketError::Flush)?;
                        Ok(())
                    }
                    0x01 => {
                        let ping = StatusPing::decode(&mut reader, ())
                            .await
                            .map_err(StatusPacketError::Ping)?;
                        info!("Received status ping: {:?}", ping);
                        writer
                            .write_packet(0x01, ping, ())
                            .await
                            .map_err(StatusPacketError::Pong)?;
                        writer.flush().await.map_err(StatusPacketError::Flush)?;
                        Ok(())
                    }
                    _ => Err(StatusPacketError::UnknownPacketId(id)),
                }
            })
            .await;

        if let Err(e) = result {
            info!("Socket {} failed to read packet: {:?}", id, e);
            break;
        }
    }
}

#[derive(Error, Debug)]
pub enum StatusPacketError<E> {
    #[error("failed to read status ping: {0}")]
    Ping(ReadExactError<E>),
    #[error("failed to write status pong: {0}")]
    Pong(WritePacketError<E, E>),
    #[error("failed to write status response: {0}")]
    Response(WritePacketError<E, E>),
    #[error("failed to flush write buffer: {0}")]
    Flush(E),
    #[error("unknown packet ID: 0x{0:02X}")]
    UnknownPacketId(i32),
}
