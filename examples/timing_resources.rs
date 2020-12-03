//! examples/timing_resources.rs

// #![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m::{asm, peripheral::DWT};
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
        rtic::pend(stm32f411::Interrupt::EXTI1);
        init::LateResources { dwt: cx.core.DWT }
    }

    #[task(binds = EXTI0, resources = [shared], priority = 2)]
    fn exti0(cx: exti0::Context) {
        asm::bkpt();
        *cx.resources.shared += 1;
    }

    #[task(binds = EXTI1, resources = [dwt, shared], priority = 1)]
    fn exti1(mut cx: exti1::Context) {
        unsafe { cx.resources.dwt.cyccnt.write(0) };
        asm::bkpt();
        rtic::pend(stm32f411::Interrupt::EXTI0);
        asm::bkpt();
        cx.resources.shared.lock(|shared| {
            // asm::bkpt();
            *shared += 1;
            // asm::bkpt();
        });
        asm::bkpt();
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
// 08000232 <EXTI0>:
//  8000232: 40 f2 00 01  	movw	r1, #0
//  8000236: ef f3 11 80  	mrs	r0, basepri
//  800023a: 00 be        	bkpt	#0
//  800023c: c2 f2 00 01  	movt	r1, #8192
//  8000240: d1 e9 00 23  	ldrd	r2, r3, [r1]
//  8000244: 01 32        	adds	r2, #1
//  8000246: 43 f1 00 03  	adc	r3, r3, #0
//  800024a: c1 e9 00 23  	strd	r2, r3, [r1]
//  800024e: 80 f3 11 88  	msr	basepri, r0
//  8000252: 70 47        	bx	lr
//
// Explain what is happening here in your own words.
//
// 1. movw (word) writes a 16-bit immediate value to the destination register. 
// 2. mrs, Move system register.
// 3. Place a break point at #0
// 4. Move top, 16x zeroes first, the add the value,#8192, "on top" (16-31) 
// 5. Load Register Dual (immediate) calculates an address from a base register value and an immediate offset.
// 6. ADDS (immediate) 1 to r2.
// 7. ADC, Add with Carry. Add value #0 with r3 to destination register r3.
// 8. STRD (Store Register Dual) calculates an address from a base register value and a register offset
// 9. mrs, Move system register back to basepri.
// 10. BX, Branch and exchange instruction set.
//
// > cargo run --example timing_resources --release --features nightly
// Then continue to the first breakpoint instruction:
// (gdb) c
// Program
//  received signal SIGTRAP, Trace/breakpoint trap.
// timing_resources::exti1 (cx=...) at examples/timing_resources.rs:39
// 39	        asm::bkpt();
//
// (gdb) x 0xe0001004
// 2
//
// (gdb) c
//  received signal SIGTRAP, Trace/breakpoint trap.
// rtic::export::run<closure-0> (priority=2, f=...) at /home/pln/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rtic-0.5.5/src/export.rs:38
//
// (gdb) x 0xe0001004
//
// 10
//
// (gdb) disassemble
//
// Dump of assembler code for function timing_resources::APP::EXTI0:
//    0x08000232 <+0>:	movw	r1, #0
//    0x08000236 <+4>:	mrs	r0, BASEPRI
// => 0x0800023a <+8>:	bkpt	0x0000
//    0x0800023c <+10>:	movt	r1, #8192	; 0x2000
//    0x08000240 <+14>:	ldrd	r2, r3, [r1]
//    0x08000244 <+18>:	adds	r2, #1
//    0x08000246 <+20>:	adc.w	r3, r3, #0
//    0x0800024a <+24>:	strd	r2, r3, [r1]
//    0x0800024e <+28>:	msr	BASEPRI, r0
//    0x08000252 <+32>:	bx	lr
// End of assembler dump.
//
// You should see that we hit the breakpoint in `exti0`, and
// that the code complies to the objdump EXTI disassembly.
//
// What was the software latency observed to enter the task?
//
// 16-2 = 14 cycles
//
// Does RTIC infer any overhead?
//
// Yes, if the typical latency is ~12 for cortex-m4, and we get 14 cycles, there are 2 cyles of overhead.
//
// The debugger reports that the breakpoint was hit in the `run<closure>`.
// The reason is that the RTIC implements the actual interrupt handler,
// from within it calls a function `run` taking the user task as a function.
//
// (Functions in Rust can be seen as closures without captured variables.)
//
// Now we can continue to measure the round trip time.
//
// (gdb) c
//
//  received signal SIGTRAP, Trace/breakpoint trap.
// timing_resources::exti1 (cx=...) at examples/timing_resources.rs:41
// 41	        asm::bkpt();
//
// (gdb) x 0xe0001004
//
// 37 
//
// You should have a total execution time in the range of 30-40 cycles.
//
// Explain the reason (for this case) that resource access in
// `exti0` was safe without locking the resource.
//
// Priority-based scheduling
//
// In `exti1` we also access `shared` but this time through a lock.
//
// (gdb) disassemble
// => 0x08000270 <+28>:	bkpt	0x0000
//    0x08000272 <+30>:	msr	BASEPRI, r0
//    0x08000276 <+34>:	movw	r0, #0
//    0x0800027a <+38>:	movt	r0, #8192	; 0x2000
//    0x0800027e <+42>:	ldrd	r2, r3, [r0]
//    0x08000282 <+46>:	adds	r2, #1
//    0x08000284 <+48>:	adc.w	r3, r3, #0
//    0x08000288 <+52>:	strd	r2, r3, [r0]
//    0x0800028c <+56>:	movs	r0, #240	; 0xf0
//    0x0800028e <+58>:	msr	BASEPRI, r0
//    0x08000292 <+62>:	bkpt	0x0000
//    0x08000294 <+64>:	msr	BASEPRI, r1
//    0x08000298 <+68>:	bx	lr
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
// You should get a value around 15 cycles.
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
//
// If you debug in vscode, just Shift-F5 to terminate session, and F5 to start debugging.
//
// If debugging in terminal you may recompile without exiting the debug session:
//
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
//
//  received signal SIGTRAP, Trace/breakpoint trap.
// timing_resources::exti1::{{closure}} (shared=<optimized out>) at examples/timing_resources.rs:43
// 43	            asm::bkpt();
//
// (gdb) x 0xe0001004
//
// [Your answer here]
//
// (gdb) c
//
//  received signal SIGTRAP, Trace/breakpoint trap.
// timing_resources::exti1::{{closure}} (shared=0x20000000 <timing_resources::APP::shared>) at examples/timing_resources.rs:45
// 45	            asm::bkpt();
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
//  received signal SIGTRAP, Trace/breakpoint trap.
// timing_resources::exti1 (cx=...) at examples/timing_resources.rs:47
//
// (gdb) x 0xe0001004
//
// [Your answer here]
//
// This is the total execution time of.
//
// - pending a task `exti0` for execution
// - preempt `exti1`
// - inside `exti0` safely access and update a shared (non atomic) resource.
// - returning to `exti1`
// - inside `exti1` safely access and update a shared (non atomic) resource
//
// Notice here, the breakpoints infer some OH and may disable
// some potential LLVM optimizations, so we obtain a "safe" (pessimistic) estimate.
//
// http://www.diva-portal.se/smash/get/diva2:1005680/FULLTEXT01.pdf
//
// You find a comparison to a typical threaded counterpart `freeRTOS` in Table 1.
//
// Give a rough estimate based on this info how long the complete task `uart1`,
// would take to execute if written in FreeRTOS. (Include the context switch, to higher
// priority task, the mutex lock/unlock in both "threads".)
//
// Motivate your answer (not just a number).
//
// [Your answer here]
//
// Notice, the Rust implementation is significantly faster than the C code version
// of Real-Time For the Masses back in 2013.
//
// Why do you think RTIC + Rust + LLVM can do a better job than hand written
// C code + Macros + gcc?
//
// (Hint, what possible optimization can safely be applied.)
//
// [Your answer here]
