#![no_main]
#![no_std]

use panic_rtt_target as _;

#[allow(unused)]
use cortex_m::asm::nop;
use rtt_target::{rprint, rprintln, rtt_init_print};

pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("RTT initialization done.");
    rprint!("Here goes a mandatory message: ");
    rprintln!("Hello, World!");

    for count in 1..=10 {
        rprintln!("count = {}", count);
        for _ in 0..20000 {
            nop();
        }
    }
    exit()
}
