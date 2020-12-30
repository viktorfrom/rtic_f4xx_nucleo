//! examples/timing_exam.rs

// #![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m::{asm, peripheral::DWT};
use panic_halt as _;
use stm32f4::stm32f411;
use rtic::cyccnt::{Instant, Duration, U32Ext};

#[rtic::app(device = stm32f411, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        #[init(0)]
        R1: u64, // non atomic data
        #[init(0)]
        R2: u64, // non atomic data
    }

    #[init(schedule = [t1, t2, t3])]
    fn init(mut cx: init::Context) {
        // Initialize (enable) the monotonic timer (CYCCNT)
        cx.core.DCB.enable_trace();
        cx.core.DWT.enable_cycle_counter();
        // cx.schedule.t1(cx.start).unwrap();
        // cx.schedule.t2(cx.start).unwrap();
        // cx.schedule.t3(cx.start).unwrap();    
    }

    // Deadline 100, Inter-arrival 100
    #[task(schedule = [t1], priority = 1)]
    fn t1(cx: t1::Context) {
        // 1) your code here to emulate timing behavior of t1

        // 2) your code here to check for overrun
        cx.schedule.t1(cx.scheduled + 100_000.cycles()).unwrap();
    }

    // Deadline 200, Inter-arrival 200
    #[task(schedule = [t2], resources = [R1, R2], priority = 2)]
    fn t2(cx: t2::Context) {
        // 1) your code here to emulate timing behavior of t2

        // 2) your code here to check for overrun
        cx.schedule.t2(cx.scheduled + 200_000.cycles()).unwrap();
    }

    // Deadline 50, Inter-arrival 50
    #[task(schedule = [t3], resources = [R2], priority = 3)]
    fn t3(cx: t3::Context) {
        // 1) your code here to emulate timing behavior of t3

        // 2) your code here to check for overrun
        cx.schedule.t3(cx.scheduled + 50_000.cycles()).unwrap();
    }

    // RTIC requires that unused interrupts are declared in an extern block when
    // using software tasks; these free interrupts will be used to dispatch the
    // software tasks.
    extern "C" {
        fn EXTI0();
        fn EXTI1();
        fn EXTI2();
    }
};

fn delay_duration(from: Instant, until: Duration)  {
    // implement a delay that busy waits for a Duration of time
    // Use `cargo doc` to generate documentation to lookup `Duration`
    // and `Instance` and corresponding operations and conversions.
    // 
    // In particular, the `elapsed` is useful.
    // Notice you can compare durations.
}

// 1) For this assignment you should first generate a task set that 
// matches the example task set from `klee_tutorial/srp_analysis/main.rs`.
//
// The task set should have the same relative timing properties as given in `main.rs`.
//
// Assume that each time unit amounts to 1_000 clock cycles, then
// the execution time of `t1` should be 10_000 clock cycles.
//
// To emulate corresponding workload you should implement `delay_duration`
// and use that to get the relative timings.
// 
// So, instead of measuring execution time of an existing application, you are to create
// one with given timing properties. 
//
// To verify that you have implemented the tasks correctly you should trigger them
// one at a time, put breakpoints at each point of interest and check the CYCCNT manually.
//
// (Verify the timing properties for each task separately.)
// 
// Commit your repository once you have done all validation.
//
// 2) 