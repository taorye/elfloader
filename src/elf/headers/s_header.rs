mod sh_flags;
mod sh_type;

pub use sh_flags::SHFlags;
pub use sh_type::SHType;

use super::SHeader;

impl SHeader {
    pub const fn section_name_offset(&self) -> usize {
        self.sh_name as usize
    }

    pub const fn section_type(&self) -> &SHType {
        &self.sh_type
    }

    pub const fn section_flags(&self) -> usize {
        self.sh_flags
    }

    pub const fn section_offset(&self) -> usize {
        self.sh_offset
    }

    pub const fn section_size(&self) -> usize {
        self.sh_size
    }

    pub const fn section_entsize(&self) -> usize {
        self.sh_entsize
    }

    pub const fn section_link(&self) -> usize {
        self.sh_link as usize
    }

    pub const fn section_info(&self) -> usize {
        self.sh_info as usize
    }

    pub const fn section_addralign(&self) -> usize {
        self.sh_addralign
    }

    pub const fn section_address(&self) -> usize {
        self.sh_addr
    }

    pub fn section_address_set(&self, addr: usize) {
        let paddr = core::ptr::addr_of!(self.sh_addr);
        unsafe {
            core::ptr::write_unaligned(paddr as *mut usize, addr);
        }
    }
}
