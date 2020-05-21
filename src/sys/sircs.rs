/// Sony Integrated Remote Control System Library 
/// This module contains the imports for the kernel's remote control routines.

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SircsData {
    pub type_: u8,
    pub cmd: u8,
    pub dev: u16,
}

sys_lib! {
    #![name = "sceSircs"]
    #![flags = 0x4001]
    #![version = (0x00, 0x00)]

    #[psp(0x71EEF62D)]
    pub unsafe fn sce_sircs_send(sd: *mut SircsData, count: i32) -> i32;
}

