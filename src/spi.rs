#[repr(C)]
/// The SPI controls on the core
///
/// Each field matches the definitions in the core specification
pub struct SpiControls {
    pub sckdiv: u32,
    pub sckmode: u32,
    _reserved_1: u32,
    _reserved_2: u32,
    pub csid: u32,
    pub csdef: u32,
    pub csmode: u32,
    _reserved_3: u32,
    _reserved_4: u32,
    _reserved_5: u32,
    pub delay0: u32,
    pub delay1: u32,
    _reserved_6: u32,
    _reserved_7: u32,
    _reserved_8: u32,
    _reserved_9: u32,
    pub fmt: u32,
    _reserved_10: u32,
    pub txdata: u32,
    pub rxdata: u32,
    pub txmark: u32,
    pub rxmark: u32,
    _reserved_11: u32,
    _reserved_12: u32,
    pub fctrl: u32,
    pub ffmt: u32,
    _reserved_13: u32,
    _reserved_14: u32,
    pub ie: u32,
    pub ip: u32,
}
/// Pointer to the QSPI0 address
pub const QSPI0: *mut SpiControls = 0x10014000 as *mut SpiControls;
/// Pointer to the SPI1 address
pub const SPI1: *mut SpiControls = 0x10024000 as *mut SpiControls;
/// Pointer to the SPI2 address
pub const SPI2: *mut SpiControls = 0x10034000 as *mut SpiControls;
