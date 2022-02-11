#![no_std]
#![doc = include_str!("../README.md")]

/// Re-export RISC-V intrinsics because they'll be useful
pub use riscv;

/// Bindings for interacting with the GPIO controller
pub mod gpio;
/// Bindings for interacting with the SPI controllers
pub mod spi;
/// Bindings for interacting with the UART controllers
pub mod uart;
