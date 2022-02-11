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
/// Pointer to the UART0 address
pub const UART0: *mut UartControls = 0x10013000 as *mut UartControls;
/// Pointer to the UART1 address
pub const UART1: *mut UartControls = 0x10023000 as *mut UartControls;
