# Some notes on Debugging usin Cortex Debug

In the `launch.json` three profiles are added.

- `Cortex Debug`, running a debug build.
- `Cortex Release`, running a release (optimized) build.
- `Cortex Nightly`, running a release build with inlined assembly (optimized).

All profiles apply to the currently active editor. (So if you e.g., try to debug this file you will get an error.) 

## Short cuts

- Shift-Control-D, to get the list of launch profiles and select the one to use,
- F5, to run the last one selected, or
- F5, continue if already started,
- Shift-F5, to abort debug session,
- F10, to step over funciton (run function till it returns),
- F11, to step into function,
- Shift-F11, to step out of function (run function till it returns),
- Ctrl-Shift-Y, focus debug console.

In the console you can give `gdb` commands directly. Error handling and "tabbing" is sub-par compared to the terminal gdb, but at least it allows you to give `gdb` commands.

It implements a history over sessions, which is neat. (Arrow up/down).

## Console

Examples of useful `gdb` console commands:

> disassemble
> continue
> break *0x8000242
> x 0xe0001004

## Peripherals

The profiles link to `STM32F401.svd`. This file is patched with to include the `DWT` unit where you find the `CYCCNT` register. You can `Right-Click` on the register to choose display format.

This can be used in the lab to observe the content of `CYCCNT` (instead of `x 0xe0001004` in the console).
