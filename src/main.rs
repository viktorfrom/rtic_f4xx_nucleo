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

// #[rtic::app(device = lm3s6965, peripherals = true)]
#[rtic::app(device = stm32f4)]
const APP: () = {
    #[init]
    fn init(cx: init::Context) {
        // Cortex-M peripherals
        let _core: cortex_m::Peripherals = cx.core;

        // Device specific peripherals
        // let _device: lm3s6965::Peripherals = cx.device;

        // // Safe access to local `static mut` variable
        // let _x: &'static mut u32 = X;
        rtt_init_print!();
        rprintln!("init");
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        //     // static mut X: u32 = 0;

        //     // // Cortex-M peripherals
        //     // let _core: cortex_m::Peripherals = cx.core;

        //     // // Device specific peripherals
        //     // // let _device: lm3s6965::Peripherals = cx.device;

        //     // // Safe access to local `static mut` variable
        //     // let _x: &'static mut u32 = X;

        rprintln!("idle");
        panic!("panic");
        loop {
            continue;
            // cortex_m::asm::wfi(); // does not work for some reason
        }
    }
};
