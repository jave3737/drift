#![no_std]
#![no_main]

use cortex_m_rt::entry;

#[entry]
fn main() -> !{
    // println!("Hello, world!");
    defmt::info!("Hello World");
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
