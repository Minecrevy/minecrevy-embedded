use embassy_executor::Spawner;
use embassy_net::{Stack, tcp::TcpSocket};
use embassy_time::Duration;

const MAX_CONNECTIONS: usize = 10;

pub fn spawn_connection_tasks(spawner: Spawner, stack: Stack<'static>) {
    for _ in 0..MAX_CONNECTIONS {
        spawner.spawn(connection_task(stack)).unwrap();
    }
}

#[embassy_executor::task(pool_size = MAX_CONNECTIONS)]
async fn connection_task(stack: Stack<'static>) {
    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];

    loop {
        let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
        socket.set_timeout(Some(Duration::from_secs(10)));
    }
}
