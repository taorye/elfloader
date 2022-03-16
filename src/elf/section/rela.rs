mod rela_type;

use core::fmt::Debug;

pub use rela_type::RelaType;

use super::Rela;

impl Rela {
    pub const fn rela_offset(&self) -> usize {
        self.r_offset
    }

    pub const fn rela_addend(&self) -> isize {
        self.r_addend
    }

    pub const fn symbol_offset(&self) -> usize {
        match () {
            #[cfg(target_pointer_width = "64")]
            () => self.r_info >> 32,
            #[cfg(target_pointer_width = "32")]
            () => self.r_info >> 8,
        }
    }

    pub const fn rela_type(&self) -> RelaType {
        unsafe { core::mem::transmute(self._rela_type()) }
    }

    const fn _rela_type(&self) -> usize {
        match () {
            #[cfg(target_pointer_width = "64")]
            () => self.r_info & 0xffffffff,
            #[cfg(target_pointer_width = "32")]
            () => self.r_info & 0xff,
        }
    }
}

impl Debug for Rela {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Rela {{ offset: {:#x}, type: {:?}, symbol: {:#x}, addend: {:#x} }}",
            self.r_offset,
            self.rela_type(),
            self.symbol_offset(),
            self.r_addend
        )
    }
}
