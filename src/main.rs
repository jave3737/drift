#![no_std]
#![no_main]

use cortex_m_rt::entry;
use hal::{clocks::Clock as _, pac, watchdog::Watchdog};
use rp2040_hal as hal;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;

#[entry]
fn main() -> ! {
    // println!("Hello, world!");
    defmt::info!("Hello World");

    let core = pac::CorePeripherals::take().unwrap();
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut &mut watchdog,
    ).ok().unwrap();
    exit();
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    defmt::error!("panicked");
    exit()
}

pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
