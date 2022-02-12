#![allow(clippy::not_unsafe_ptr_arg_deref)]

use bit_field::BitField;
use core::ptr::read_volatile;

#[repr(C)]
/// The UART controls on the core
///
/// Each field matches the definitions in the core specification. They are exposed so that the end
/// user can work with them. However, if the behavior you want is covered by the methods provided
/// for this class, you should use those
pub struct UartControls {
    pub txdata: u32,
    pub rxdata: u32,
    pub txctl: u32,
    pub rxctl: u32,
    pub ie: u32,
    pub ip: u32,
    pub div: u32,
}

macro_rules! bit_write {
    ($( ( #[$outer:meta], $name:ident, $field:ident, $index:expr, $value:expr ) ),+) => {$(
        #[inline]
        #[$outer]
        pub fn $name (uart: *mut UartControls) {
            unsafe { (*uart).$field.set_bit($index, $value) };
        }
    )*}
}

macro_rules! bit_read {
    ($( ( #[$outer:meta], $name:ident, $field:ident, $index:expr ) ),+) => {$(
        #[inline]
        #[$outer]
        pub fn $name (uart: *const UartControls) -> bool {
            unsafe { read_volatile(&(*uart).$field) }.get_bit($index)
        }
    )*}
}

// These impls would be better using self than some other name for it, but
// we need to take in the pointer so that we can safely work with the constants
// that we expose (no global constants that are `&mut` afaict).
//
// Once `arbitrary_self_types` is stabilized, we'll be able to do that (see
// here: https://github.com/rust-lang/rust/issues/44874).
impl UartControls {
    /// Attempt to read a byte, returning `Some` if a byte is immediately available for reading and
    /// a `None` if no bytes are immediately available for reading.
    #[inline]
    pub fn poll_read_byte(uart: *const UartControls) -> Option<u8> {
        let x = unsafe { read_volatile(&(*uart).rxctl) };
        if x.get_bit(31) {
            Some(x as u8)
        } else {
            None
        }
    }

    #[inline]
    // Set the baud rate divisor to the specified value
    pub fn set_baud_rate_divisor(uart: *mut UartControls, value: u16) {
        unsafe {
            (*uart).div.set_bits(0..16, value as u32);
        }
    }

    bit_read!(
    (
        #[doc = "Returns true if there's a pending interrupt for the transmit queue length going below the watermark."],
        transmit_interrupt_pending,
        ip,
        0
    ),
    (
        #[doc = "Returns true if there's a pending interrupt for the receive queue length going above the watermark."],
        receive_interrupt_pending,
        ip,
        1
    ),
    (
        #[doc = "Returns true if interrupts are enabled for the transmit queue length going below the watermark."],
        transmit_interrupt_enable,
        txctl,
        0
    ),
    (
        #[doc = "Returns true if interrupts are enabled for the receive queue length going above the watermark."],
        receive_interrupt_enable,
        rxctl,
        0
    ),
    (
        #[doc = "Returns true if the transmit queue is full."],
        transmit_queue_full,
        txdata,
        31
    )
    );
    bit_write!(
    (
        #[doc = "Enable interrupts for the receive queue length going above the watermark."],
        receive_enable_interrupts,
        rxctl,
        0,
        true
    ),
    (
        #[doc = "Disable interrupts for the receive queue length going above the watermark."],
        receive_disable_interrupts,
        rxctl,
        0,
        false
    ),
    (
        #[doc = "Enable interrupts for the transmit queue length going below the watermark."],
        transmit_enable_interrupts,
        txctl,
        0,
        true
    ),
    (
        #[doc = "Disable interrupts for the transmit queue length going below the watermark."],
        transmit_disable_interrupts,
        txctl,
        0,
        false
    ),
    (
        #[doc = "Set to true for two stop bits, set to false for one stop bit."],
        set_num_stop_bits,
        txctl,
        1,
        false
    )
    );
}

/// Pointer to the UART0 address
pub const UART0: *mut UartControls = 0x10013000 as *mut UartControls;
/// Pointer to the UART1 address
pub const UART1: *mut UartControls = 0x10023000 as *mut UartControls;
