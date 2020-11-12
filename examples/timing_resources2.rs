//! examples/timing_resources.rs

// #![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m::peripheral::DWT;
//use cortex_m::{asm, peripheral::DWT};
use panic_halt as _;
use stm32f4::stm32f411;

#[rtic::app(device = stm32f411)]
const APP: () = {
    struct Resources {
        dwt: DWT,

        #[init(0)]
        shared: u64, // non atomic data
    }

    #[init]
    fn init(mut cx: init::Context) -> init::LateResources {
        // Initialize (enable) the monotonic timer (CYCCNT)
        cx.core.DCB.enable_trace();
        cx.core.DWT.enable_cycle_counter();
        init::LateResources { dwt: cx.core.DWT }
    }

    #[idle(resources = [shared])]
    fn idle(_cx: idle::Context) -> ! {
        // unsafe { cx.resources.dwt.cyccnt.write(0) };
        // // asm::bkpt();
        // rtic::pend(stm32f411::Interrupt::EXTI0);
        // // asm::bkpt();
        // cx.resources.shared.lock(|shared| {
        //     // asm::bkpt();
        //     *shared += 1;
        //     // asm::bkpt();
        // });
        // asm::bkpt();
        loop {
            continue;
        }
    }

    #[task(binds = EXTI0, resources = [shared], priority = 2)]
    fn exti0(cx: exti0::Context) {
        // asm::bkpt();
        *cx.resources.shared += 1;
    }

    #[task(binds = EXTI1, resources = [dwt, shared], priority = 1)]
    fn exti1(mut cx: exti1::Context) {
        unsafe { cx.resources.dwt.cyccnt.write(0) };
        // asm::bkpt();
        rtic::pend(stm32f411::Interrupt::EXTI0);
        // asm::bkpt();
        cx.resources.shared.lock(|shared| {
            // asm::bkpt();
            *shared += 1;
            // asm::bkpt();
        });
        // asm::bkpt();
    }
};

// Now we are going to have a look at the resource management of RTIC.
//
// First create an objdump file:
// >  cargo objdump --example timing_resources --release  --features nightly -- --disassemble > timing_resources.objdump
//
// Lookup the EXTI0 symbol (RTIC binds the exti0 task to the interrupt vector).
//
// You should find something like:
//
// 080002b6 <EXTI0>:
//  80002b6: 40 f2 00 00  	movw	r0, #0
//  80002ba: 00 be        	bkpt	#0
//  80002bc: c2 f2 00 00  	movt	r0, #8192
//  80002c0: d0 e9 00 12  	ldrd	r1, r2, [r0]
//  80002c4: 01 31        	adds	r1, #1
//  80002c6: 42 f1 00 02  	adc	r2, r2, #0
//  80002ca: c0 e9 00 12  	strd	r1, r2, [r0]
//  80002ce: 00 20        	movs	r0, #0
//  80002d0: 80 f3 11 88  	msr	basepri, r0
//  80002d4: 70 47        	bx	lr
//
// Explain what is happening here in your own words.
//
// [Your code here]
//
// > cargo run --example timing_resources --release --features nightly
// Then continue to the first breakpoint instruction:
// (gdb) c
// timing_resources::idle (cx=...) at examples/timing_resources.rs:32
// 32              asm::bkpt();
//
// (gdb) x 0xe0001004
// 0
//
// (gdb) c
// timing_resources::exti0 (cx=...) at examples/timing_resources.rs:44
// 44              asm::bkpt();
//
// (gdb) x 0xe0001004
//
// [Your answer here]
//
// (gdb) disassemble
//
// [Your answer here]
//
// You should see that we hit the breakpoint in `exti0`, and
// that the code complies to the objdump EXTI disassembly.
//
// What was the software latency observed to enter the task?
//
// [Your answer here]
//
// Does RTIC infer any overhead?
//
// [Your answer here]
//
// Now we can continue to measure the round trip time.
//
// (gdb) c
//
// (gdb) x 0xe0001004
// timing_resources::idle (cx=...) at examples/timing_resources.rs:34
// 34              asm::bkpt();
//
// [Your answer here]
//
// You should have a total execution time in the range of 30 cycles.
//
// Explain the reason (for this case) that resource access in
// `exti0` was safe without locking the resource.
//
// [Your answer here]
//
// In `idle` we also access `shared` but this time through a lock.
//
// (gdb) disassemble
// => 0x0800026e <+26>:    bkpt    0x0000
//    0x08000270 <+28>:    ldrb    r2, [r0, #0]
//    0x08000272 <+30>:    cbz     r2, 0x800028c <timing_resources::idle+56>
//    0x08000274 <+32>:    movw    r0, #0
//    0x08000278 <+36>:    movt    r0, #8192       ; 0x2000
//    0x0800027c <+40>:    ldrd    r1, r2, [r0]
//    0x08000280 <+44>:    adds    r1, #1
//    0x08000282 <+46>:    adc.w   r2, r2, #0
//    0x08000286 <+50>:    strd    r1, r2, [r0]
//    0x0800028a <+54>:    b.n     0x80002b2 <timing_resources::idle+94>
//    0x0800028c <+56>:    movs    r2, #1
//    0x0800028e <+58>:    movw    r12, #0
//    0x08000292 <+62>:    strb    r2, [r0, #0]
//    0x08000294 <+64>:    movs    r2, #240        ; 0xf0
//    0x08000296 <+66>:    msr     BASEPRI, r2
//    0x0800029a <+70>:    movt    r12, #8192      ; 0x2000
//    0x0800029e <+74>:    ldrd    r3, r2, [r12]
//    0x080002a2 <+78>:    adds    r3, #1
//    0x080002a4 <+80>:    adc.w   r2, r2, #0
//    0x080002a8 <+84>:    strd    r3, r2, [r12]
//    0x080002ac <+88>:    msr     BASEPRI, r1
//    0x080002b0 <+92>:    strb    r1, [r0, #0]
//    0x080002b2 <+94>:    bkpt    0x0000
//
// We can now execute the code to the next breakpoint to get the
// execution time of the lock.
//
// (gdb) c
// timing_resources::idle (cx=...) at examples/timing_resources.rs:36
// 36              asm::bkpt();
//
// (gdb) x 0xe0001004
//
// [Your answer here]
//
// Calculate the total time (in cycles), for this section of code.
//
// [Your answer here]
//
// You should get a value around 25 cycles.
//
// Now look at the "critical section", i.e., how many cycles
// are the lock held?
// To this end you need to insert `asm::bkpt()` on entry and exit
// inside the closure.
//
// cx.resources.shared.lock(|shared| {
//     asm::bkpt();
//     *shared += 1;
//     asm::bkpt();
// });
//
// Change the code, and compile it from withing gdb
// (gdb) shell cargo build --example timing_resources --release  --features nightly
//   Compiling app v0.1.0 (/home/pln/courses/e7020e/app)
//     Finished release [optimized + debuginfo] target(s) in 0.32s
//
// and load the newly compiled executable:
// (gdb) load
// ...
// Transfer rate: 1 KB/sec, 406 bytes/write.
//
// Now you can continue until you hit the first breakpoint in the lock closure.
//
// (gdb) c
// rtic::export::lock<u64,(),closure-0> (ptr=<optimized out>, priority=0x2000ffef, ceiling=1, nvic_prio_bits=4, f=...) at /home/pln/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-0.6.4/src/asm.rs:11
// 11              () => unsafe { llvm_asm!("bkpt" :::: "volatile") },
//
// (gdb) x 0xe0001004
//
// [Your answer here]
//
// (gdb) c
// rtic::export::lock<u64,(),closure-0> (ptr=<optimized out>, priority=0x2000ffef, ceiling=1, nvic_prio_bits=4, f=...) at /home/pln/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-0.6.4/src/asm.rs:11
// 11              () => unsafe { llvm_asm!("bkpt" :::: "volatile") },
//
// (gdb) x 0xe0001004
//
// [Your answer here]
//
// From a real-time perspective the critical section infers
// blocking (of higher priority tasks).
//
// How many clock cycles is the blocking?
//
// [Your answer here]
//
// Finally continue out of the closure.
//
// (gdb) c
// timing_resources::idle (cx=...) at examples/timing_resources.rs:40
// 40              asm::bkpt();
//
// (gdb) x 0xe0001004
//
// [Your answer here]
//
// This is the total execution time of.
//
// - pending a task `exti` for execution
// - preempt `idle`
// - inside `exti` safely access and update a shared (non atomic resource).
// - returning to `idle`
// - safely access and update a shared (non atomic) resource
//
// Notice here, the breakpoints infer some OH and may disable
// some potential LLVM optimizations.
//
//
