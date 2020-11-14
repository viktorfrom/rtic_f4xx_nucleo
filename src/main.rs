//! examples/init.rs

#![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m;
// use panic_halt as _;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4;
#[rtic::app(device = stm32f4, peripherals = false)]
mod app {
    use super::*;
    #[init]
    fn init(_cx: init::Context) -> init::LateResources {
        rtt_init_print!();
        rprintln!("init");
        init::LateResources {}
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        rprintln!("idle");
        panic!("panic");
        loop {
            continue;
        }
    }
}
