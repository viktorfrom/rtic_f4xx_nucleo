//! main.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_halt as _;
//use panic_rtt_target as _;
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
        panic!("panic");
        //loop {
        //    continue;
        //}
    }
};

// A) A Simple Trace
// > cargo run
// cargo run
//    Compiling app v0.1.0 (/home/pln/courses/d7020e/rtic_f4xx_nucleo)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.18s
//      Running `probe-run --chip STM32F411RETx target/thumbv7em-none-eabi/debug/app`
//   (HOST) INFO  flashing program (15.06 KiB)
//   (HOST) INFO  success!
// ────────────────────────────────────────────────────────────────────────────────
// init
// idle
//
// B) Breaking
// Now press Ctrl-C
// ^Cstack backtrace:
//    0: app::idle
//         at src/main.rs:25
//    1: main
//         at src/main.rs:13
//    2: Reset
//         at /home/pln/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.13/src/lib.rs:526
//
// Make sure you got the expected output
//
// C) Panic tracing
// Rust is designed for reliability, with the aim to deliver memory safety
// and defined behavior at all times.
// Recoverable errors (in libraries and user code) should use the `Result<T,E>` type,
// while unrecoverable errors should `panic`.
//
// Let's introduce a `panic` (uncomment line 24).
// > cargo run
//
// What is the output?
//
// 24 |           panic!("panic");                                              
//    |       ---------------- any code following this expression is unreachable
// 25 | /         loop {
// 26 | |             continue; 
// 27 | |         }                                                
//    | |_________^ unreachable expression 
//
// D) Panic halt
// Tracing is nice during development, but requires a debugger attached
// and a host listening. For a deployed product, other `panic` behavior
// should be adopted (e.g. storing to flash, for later post-mortem debugging)
// or just reset:ing the device. In this example we chose just to `halt`
//
// Enable `panic_halt` (line 8).
// > cargo run
//
// What is the output?
//
// ────────────────────────────────────────────────────────────────────────────────
// init
// idle
//
// Now press Ctrl-C
//
// What is the output?
//
// ^Cstack backtrace:
//    0: core::sync::atomic::compiler_fence
//         at /home/from/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/sync/atomic.rs:2766
//  1: rust_begin_unwind
//        at /home/from/.cargo/registry/src/github.com-1ecc6299db9ec823/panic-halt-0.2.0/src/lib.rs:33
//   2: core::panicking::panic_fmt
//        at /rustc/04488afe34512aa4c33566eb16d8c912a3ae04f9/src/libcore/panicking.rs:85
//   3: core::panicking::panic
//        at /rustc/04488afe34512aa4c33566eb16d8c912a3ae04f9/src/libcore/panicking.rs:50
//   4: app::idle
//        at src/main.rs:24
//   5: main
//        at src/main.rs:13
//   6: Reset
//        at /home/from/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.13/src/lib.rs:526
//
// E) Find the source
// Figure out how to find the source of `panic_halt`, and look at the implementation.
//
// - `cargo doc --open` (you need to disable the rtt-panic handler in `Cargo.toml`).
// - `crates.io`
//
// Paste the implementation here
// [Your answer here]
