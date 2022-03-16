mod rela;
mod symbol;

pub use rela::RelaType;
pub use symbol::{STBind, STType, STVis};

#[repr(C)]
pub struct Rela {
    pub r_offset: usize,
    pub r_info: usize,
    pub r_addend: isize,
}

#[cfg(target_pointer_width = "32")]
#[repr(C)]
pub struct Symbol {
    pub st_name: u32,
    pub st_value: usize,
    pub st_size: usize,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
}

#[cfg(target_pointer_width = "64")]
#[repr(C)]
pub struct Symbol {
    pub st_name: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
    pub st_value: usize,
    pub st_size: usize,
}
