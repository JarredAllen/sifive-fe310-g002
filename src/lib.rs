#![no_std]

#[repr(C)]
/// The GPIO controls on the core
///
/// Each field matches the definitions in the core specification
pub struct GpioControls {
    pub input_val: u32,
    pub input_en: u32,
    pub output_en: u32,
    pub output_val: u32,
    pub pue: u32,
    pub ds: u32,
    pub rise_ie: u32,
    pub rise_ip: u32,
    pub fall_ie: u32,
    pub fall_ip: u32,
    pub high_ie: u32,
    pub high_ip: u32,
    pub low_ie: u32,
    pub low_ip: u32,
    pub out_xor: u32,
}

pub const GPIO: *mut GpioControls = 0x1001_2000 as *mut GpioControls;

#[repr(C)]
/// The UART controls on the core
///
/// Each field matches the definitions in the core specification
pub struct UartControls {
    pub txdata: u32,
    pub rxdata: u32,
    pub txctl: u32,
    pub rxctl: u32,
    pub ie: u32,
    pub ip: u32,
    pub div: u32,
}
pub const UART0: *mut UartControls = 0x10013000 as *mut UartControls;
pub const UART1: *mut UartControls = 0x10023000 as *mut UartControls;
