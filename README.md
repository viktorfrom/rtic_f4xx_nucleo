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



