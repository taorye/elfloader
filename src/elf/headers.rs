mod e_header;
mod s_header;

use e_header::{EIdent, EMachine, EType, EVersion};
pub use s_header::{SHFlags, SHType};

#[repr(C)]
pub struct EHeader {
    /* byte 0-15 */
    pub e_ident: EIdent, /* Magic number and other info */
    /* byte 16-17 */
    pub e_type: EType, /* Object file type */
    /* byte 18-19 */
    pub e_machine: EMachine, /* Architecture */
    /* byte 19-22 */
    pub e_version: EVersion, /* Object file version */
    pub e_entry: usize,      /* Entry point virtual address */
    pub e_phoff: usize,      /* Program header table file offset */
    pub e_shoff: usize,      /* Section header table file offset */
    pub e_flags: u32,        /* Processor-specific flags */
    pub e_ehsize: u16,       /* ELF header size in bytes */
    pub e_phentsize: u16,    /* Program header table entry size */
    pub e_phnum: u16,        /* Program header table entry count */
    pub e_shentsize: u16,    /* Section header table entry size */
    pub e_shnum: u16,        /* Section header table entry count */
    pub e_shstrndx: u16,     /* Section header string table index */
}

#[repr(C)]
#[derive(Debug)]
pub struct SHeader {
    pub sh_name: u32,
    pub sh_type: SHType,
    pub sh_flags: usize,
    pub sh_addr: usize,
    pub sh_offset: usize,
    pub sh_size: usize,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: usize,
    pub sh_entsize: usize,
}
