#![allow(clippy::not_unsafe_ptr_arg_deref)]

#[repr(C)]
/// The Clint controls on the core
///
/// Each field matches the definitions in the specification. They are exposed so that the end user
/// can work with them. However, if the behavior you want is covered by the methods provided for
/// this class, you should use those.
///
/// Note that ClintControls should only be used by the pointer provided by this module, [CLINT].
///
/// This struct contains definitions for the FE310-G002 core specifically, which has only one hart.
/// Thus, the structure is simpler than it would be for a multi-hart system.
pub struct ClintControls {
    /// This register is mostly used for inter-hart communication, but the FE310 only has one hart,
    /// so it is unlikely to be useful.
    pub msip: u32,
    _reserved_1: [u32; 0xFFF],
    pub mtimecmp: u32,
    _reserved_2: [u32; 0xFFF],
    pub mtime: u32,
    _reserved_3: [u32; 0xFFF],
}

impl ClintControls {
    /// Sets the value in `mtime` to be zero
    #[inline]
    pub fn clear_mtime(clint: *mut ClintControls) {
        unsafe { (*clint).mtime = 0 };
    }

    /// Sets the value in `mtimecmp` to be the target value.
    ///
    /// You will likely want to pair this with calling [ClintControls::clear_mtime] to get the
    /// mtime register into a known state such that you know how long it will be until the
    /// interrupt activates.
    #[inline]
    pub fn set_mtimecomp(clint: *mut ClintControls, target: u32) {
        unsafe { (*clint).mtimecmp = target };
    }
}

pub const CLINT: *mut ClintControls = 0x20000000 as *mut ClintControls;
