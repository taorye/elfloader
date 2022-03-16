mod st_bind;
mod st_type;
mod st_vis;

use core::fmt::Debug;

pub use st_bind::STBind;
pub use st_type::STType;
pub use st_vis::STVis;

use super::Symbol;

impl Symbol {
    pub const fn symbol_value(&self) -> usize {
        self.st_value
    }

    pub fn symbol_value_set(&mut self, v: usize) {
        self.st_value = v;
    }

    pub const fn symbol_size(&self) -> usize {
        self.st_size
    }

    pub const fn symbol_name_offset(&self) -> usize {
        self.st_name as usize
    }

    pub const fn symbol_section_ndx(&self) -> usize {
        self.st_shndx as usize
    }

    pub const fn symbol_type(&self) -> STType {
        unsafe { core::mem::transmute(self.st_info & 0xf) }
    }

    pub const fn symbol_bind(&self) -> STBind {
        unsafe { core::mem::transmute(self.st_info >> 4) }
    }

    pub const fn symbol_visibility(&self) -> STVis {
        unsafe { core::mem::transmute(self.st_other & 0x3) }
    }
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Symbol {{ value: {:#x}, size: {}, type: {:?}, bind: {:?}, vis: {:?}, ndx: {}, name: {:#x} }}",
            self.st_value,
            self.st_size,
            self.symbol_type(),
            self.symbol_bind(),
            self.symbol_visibility(),
            self.symbol_section_ndx(),
            self.symbol_name_offset()
        )
    }
}
