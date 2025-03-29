use embassy_executor::Spawner;
use embassy_net::{Config, Ipv4Cidr, StackResources, StaticConfigV4, tcp::TcpSocket};
use embassy_net_tuntap::TunTapDevice;
use embassy_time::Duration;
use embedded_byteorder::ReadExactError;
use heapless::Vec;
use minecrevy_encdec::{
    AsyncDecode, AsyncReadMinecraftExt, AsyncWriteMinecraftExt, WritePacketError,
};
use minecrevy_protocol::r770::{
    Handshake, NextState, StatusPing, StatusRequest, StatusResponse, StatusResponsePlayers,
    StatusResponseSimple,
};
use rand::RngCore;
use static_cell::StaticCell;
use thiserror::Error;

extern crate embassy_time_std;

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, TunTapDevice>) -> ! {
    runner.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let device = TunTapDevice::new("tap99").unwrap();

    let gateway = [192, 168, 69, 1].into();
    let static_config = StaticConfigV4 {
        address: Ipv4Cidr::new([192, 168, 69, 2].into(), 24),
        gateway: Some(gateway),
        dns_servers: Vec::new(),
    };
    let config = Config::ipv4_static(static_config);

    static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
    let (stack, runner) = embassy_net::new(
        device,
        config,
        RESOURCES.init(StackResources::new()),
        rand::rng().next_u64(),
    );
    spawner.spawn(net_task(runner)).unwrap();

    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];

    loop {
        let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
        socket.set_timeout(Some(Duration::from_secs(10)));

        println!("Listening on TCP:25565...");
        if let Err(e) = socket.accept(25565).await {
            println!("accept error: {:?}", e);
            continue;
        }

        println!("Received connection from {:?}", socket.remote_endpoint());

        let Ok(handshake): Result<
            Handshake,
            minecrevy_encdec::ReadPacketError<embassy_net::tcp::Error, _>,
        > = socket
            .read_packet(async move |id, mut reader| {
                assert_eq!(id, 0x00);
                Handshake::decode(&mut reader, ()).await
            })
            .await
        else {
            println!("handshake error");
            continue;
        };

        if handshake.protocol_version == 770 {
            println!("Received handshake: {:?}", handshake);
        } else {
            println!(
                "Received unsupported protocol version: {}",
                handshake.protocol_version
            );
            continue;
        }

        if handshake.next_state != NextState::Status {
            println!(
                "Received unsupported next state: {:?}",
                handshake.next_state
            );
            continue;
        }

        handle_status_packets(socket).await;
    }
}

const STATUS_RESPONSE: StatusResponse<'static> = StatusResponse {
    version: minecrevy_protocol::r770::Version::V1_21_5,
    players: StatusResponsePlayers { max: 0, online: 0 },
    description: "Hello, world!",
    enforces_secure_chat: false,
};

const STATUS_RESPONSE_SIMPLE: StatusResponseSimple<'static> = StatusResponseSimple(
    r#"{"version":{"name":"1.21.5","protocol":770},"players":{"max":0,"online":0},"description":"Hello, world!","enforcesSecureChat":false}"#,
);

async fn handle_status_packets(mut socket: TcpSocket<'_>) {
    loop {
        let (mut reader, mut writer) = socket.split();

        if let Err(e) = reader
            .read_packet(async move |id, mut reader| {
                match id {
                    0x00 => {
                        let Ok(request) = StatusRequest::decode(&mut reader, ()).await;
                        println!("Received status request: {request:?}");
                        writer
                            .write_packet(0x00, STATUS_RESPONSE_SIMPLE, ())
                            .await
                            .map_err(StatusPacketError::Response)?;
                        println!("Flushing response...");
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
                        println!("Received status ping: {ping:?}");
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
            .await
        {
            println!("Error reading packet: {:?}", e);
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
