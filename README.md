# RTIC on the STM32F4xx Nucleo board

## Rust

We assume Rust to be installed using [rustup](https://www.rust-lang.org/tools/install).

Additionally you need to install the `thumbv7em-none-eabi` target.
```shell
> rustup target add thumbv7em-none-eabi 
```

You also need [cargo-binutils](https://github.com/rust-embedded/cargo-binutils).

## For RTT tracing

We assume the following tools are in place:

- [probe-run](https://crates.io/crates/probe-run)

## For low level `gdb` based debugging

Linux tooling:

- `openocd`
- `arm-none-eabi-gdb`, or
- `gdb-multiarch`

## Editor

You may use any editor of choice. `vscode` supports Rust using the  `rust-analyzer` plugin.

---

## Exercises

- `src/main.rs`

  Developing embedded applications in Rust is made simle by the RTIC framework. In this exercise you will familiarize yourself with the basics `init` and `idle`, and see how you can trace the output to a terminal using `cargo-run`.

  You will also learn about `panic`s and how they can be traced.

- `examples/rtt_timing.rs`

  Here you will learn about cycle accurate timing measurements.

  - Using instrumentation code (which introduces bloat and overhead).

  - Non intrusive measurements using the on-chip debug unit and `gdb`.

  - Code generation optimization

  - Code inspection, `objdump`, debugging and interactive `disassemble`.

  - Code trimming, RTIC is "A Zero-Cost Abstraction for Memory Safe Concurrency".
