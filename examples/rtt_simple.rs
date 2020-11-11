//! examples/rtt_simple.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4;

#[rtic::app(device = stm32f4)]
const APP: () = {
    #[init]
    fn init(_cx: init::Context) {
        rtt_init_print!();
        rprintln!("init");
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        rprintln!("idle");
        loop {
            continue;
        }
    }
};

// > cargo run rtt_simple
//    Compiling app v0.1.0 (/home/pln/courses/e7020e/app)
// Finished dev [unoptimized + debuginfo] target(s) in 0.15s
// Running `probe-run --chip STM32F411RETx target/thumbv7em-none-eabi/debug/examples/rtt_simple`
// flashing program ..
// DONE
// resetting device
// init
// idle
//
// > [Ctrl-C]
// stack backtrace:
// 0: 0x0800031a - rtt_simple::idle
// 1: 0x0800036c - main
// 2: 0x080001da - Reset
