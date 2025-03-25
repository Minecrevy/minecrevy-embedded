#![no_std]
#![no_main]

use core::str::FromStr;

use cyw43::JoinOptions;
use cyw43_pio::{DEFAULT_CLOCK_DIVIDER, PioSpi};
use defmt::{info, unwrap, warn};
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_net::{Config, DhcpConfig, StackResources, tcp::TcpSocket};
use embassy_rp::{
    bind_interrupts,
    gpio::{Level, Output},
    peripherals::{DMA_CH0, PIO0, TRNG},
    pio::{InterruptHandler, Pio},
    trng::Trng,
};
use embassy_time::{Duration, Timer};
use heapless::String;
use minecrevy_bytes::AsyncReadMinecraftExt;
use minecrevy_encdec::AsyncDecode;
use minecrevy_protocol::r769::Handshake;
use panic_halt as _;
use static_cell::StaticCell;

// Program metadata for `picotool info`.
// This isn't needed, but it's recommended to have these minimal entries.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"Minecrevy"),
    embassy_rp::binary_info::rp_program_description!(
        c"A baremetal Minecraft server written in Rust with Embassy."
    ),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
    TRNG_IRQ => embassy_rp::trng::InterruptHandler<TRNG>;
});

const WIFI_NETWORK: Option<&str> = option_env!("WIFI_NETWORK");
const WIFI_PASSWORD: Option<&str> = option_env!("WIFI_PASSWORD");

#[embassy_executor::task]
async fn cyw43_task(
    runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>,
) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, cyw43::NetDriver<'static>>) -> ! {
    runner.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let fw = include_bytes!("../cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../cyw43-firmware/43439A0_clm.bin");

    let pwr = Output::new(p.PIN_23, Level::Low);
    let cs = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        DEFAULT_CLOCK_DIVIDER,
        pio.irq0,
        cs,
        p.PIN_24,
        p.PIN_29,
        p.DMA_CH0,
    );

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
    unwrap!(spawner.spawn(cyw43_task(runner)));

    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    let mut trng = Trng::new(p.TRNG, Irqs, embassy_rp::trng::Config::default());
    let seed = trng.blocking_next_u64();

    let mut dhcp_config = DhcpConfig::default();
    dhcp_config.hostname = Some(unwrap!(String::from_str("Minecrevy")));
    let config = Config::dhcpv4(dhcp_config);

    static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
    let (stack, runner) = embassy_net::new(
        net_device,
        config,
        RESOURCES.init(StackResources::new()),
        seed,
    );
    unwrap!(spawner.spawn(net_task(runner)));

    loop {
        match control
            .join(
                WIFI_NETWORK.unwrap(),
                JoinOptions::new(WIFI_PASSWORD.unwrap().as_bytes()),
            )
            .await
        {
            Ok(_) => break,
            Err(e) => {
                info!("Failed to join WiFi network (status={})", e.status);
            }
        }
    }

    info!("Waiting for DHCP...");
    while !stack.is_config_up() {
        Timer::after_millis(100).await;
    }
    info!("DHCP is now up!");

    control.gpio_set(0, true).await;
    Timer::after_millis(250).await;
    control.gpio_set(0, false).await;

    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];

    loop {
        let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
        socket.set_timeout(Some(Duration::from_secs(10)));

        control.gpio_set(0, false).await;
        info!("Listening on TCP:25565...");
        if let Err(e) = socket.accept(25565).await {
            warn!("accept error: {:?}", e);
            continue;
        }

        info!("Received connection from {:?}", socket.remote_endpoint());

        let Ok(Ok(handshake)) = socket
            .read_packet(move |id, mut reader| async move {
                assert_eq!(id, 0x00);
                Handshake::decode(&mut reader, ()).await
            })
            .await
        else {
            continue;
        };

        if handshake.protocol_version == 769 {
            info!("Received handshake: {:?}", handshake);
            control.gpio_set(0, true).await;
            Timer::after_millis(1000).await;
        } else {
            warn!(
                "Received unsupported protocol version: {}",
                handshake.protocol_version
            );
        }
    }
}
