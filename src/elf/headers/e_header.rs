mod e_ident;
mod e_machine;
mod e_type;
mod e_version;

use core::fmt::Debug;

pub use e_ident::EIdent;
pub use e_machine::EMachine;
pub use e_type::EType;
pub use e_version::EVersion;

use super::EHeader;

impl EHeader {
    pub const fn is_valid(&self) -> bool {
        self.e_ident.is_valid()
            && match self.e_version {
                EVersion::CURRENT => true,
                _ => false,
            }
    }

    pub const fn elf_flags(&self) -> u32 {
        self.e_flags
    }

    pub const fn section_header_offset(&self) -> usize {
        self.e_shoff
    }

    pub const fn section_header_nums(&self) -> usize {
        self.e_shnum as usize
    }

    pub const fn shstrndx(&self) -> usize {
        self.e_shstrndx as usize
    }
}

impl Debug for EHeader {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("EHeader")
            .field("ident", &self.e_ident)
            .field("type", &self.e_type)
            .field("machine", &self.e_machine)
            .field("version", &self.e_version)
            .field("entry", &self.e_entry)
            .field("phoff", &self.e_phoff)
            .field("shoff", &self.e_shoff)
            .field("flags", &self.e_flags)
            .field("ehsize", &self.e_ehsize)
            .field("phentsize", &self.e_phentsize)
            .field("phnum", &self.e_phnum)
            .field("shentsize", &self.e_shentsize)
            .field("shnum", &self.e_shnum)
            .field("shstrndx", &self.e_shstrndx)
            .finish()
    }
}
