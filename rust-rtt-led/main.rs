// cargo run --release
// or
// cargo run --release --features=rtt
#![no_main]
#![no_std]

#[cfg(feature = "rtt")]
use panic_rtt_target as _;
use ra4m1 as pac;
mod setting;

#[allow(unused)]
use cortex_m::asm::nop;

#[cfg(feature="rtt")]
use rtt_target::{rprint, rprintln, rtt_init_print};

#[cfg(not(feature="rtt"))]
macro_rules! rtt_init_print { () => { } }
#[cfg(not(feature="rtt"))]
macro_rules! rprint { ($($arg:tt)*) => { let _ = ($($arg)*); } }
#[cfg(not(feature="rtt"))]
macro_rules! rprintln { ($($arg:tt)*) => { let _ = ($($arg)*); } }
#[cfg(not(feature = "rtt"))]
#[inline(never)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    cortex_m::asm::udf();
}

#[inline(never)]
pub fn exit() -> ! {
    cortex_m::asm::bkpt();
    loop { }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("RTT initialization done.");
    rprint!("Here goes a mandatory message: ");
    rprintln!("Hello, World!");

    let device = unsafe { pac::Peripherals::steal() };

    let led_pin_bit = 0x0800;
    // LED pin to Output.
    device.PORT1.pdr().write(|w| unsafe { w.pdr().bits(led_pin_bit) });

    for count in 1..=10 {
        rprintln!("count = {}", count);
        // LED pin to High or Low.
        let podr = if count & 1 != 0 { led_pin_bit } else { 0 };
        device.PORT1.podr().write(|w| unsafe { w.podr().bits(podr) });
        for _ in 0..50000 {
            nop();
        }
    }

    exit()
    // probe-run seems to reset the mcu when exiting.
    // so you will observe another session of led blinks.
}
