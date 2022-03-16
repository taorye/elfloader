pub mod headers;
pub mod section;

use alloc::vec::Vec;
use core::ptr;
use core::slice;

use headers::{EHeader, SHeader};
use headers::{SHFlags, SHType};

use section::{Rela, Symbol};
use section::{RelaType, STBind, STType, STVis};

#[derive(Debug)]
pub struct ELFFile {
    ehaddr: *const EHeader,
    shaddr: *const [SHeader],
}

#[derive(Debug)]
pub enum ELFFileError {
    FileNotFound,
    FileHasNotSection,
    FileNotValid,
}

impl ELFFile {
    pub fn parse(elf_bin: *const u8) -> Result<ELFFile, ELFFileError> {
        use self::ELFFileError::*;
        if elf_bin.is_null() {
            return Err(FileNotFound);
        }
        let elf_file = {
            let ehaddr = elf_bin as *const EHeader;
            let shaddr = unsafe {
                let ehdr = &*ehaddr;
                let shnum = ehdr.e_shnum as usize;
                if shnum < 1 {
                    return Err(FileHasNotSection);
                }
                let shoff = ehdr.e_shoff;
                let shdr = (ehaddr as *const u8).offset(shoff as isize) as *const SHeader;
                ptr::addr_of!(*slice::from_raw_parts(shdr, shnum))
            };
            ELFFile { ehaddr, shaddr }
        };
        if !elf_file.elf_header().is_valid() {
            return Err(FileNotValid);
        }
        Ok(elf_file)
    }

    pub fn get_undefined_symbol_names(&self) -> Vec<&'static str> {
        self.section_headers()
            .iter()
            .filter(|&sh| matches!(sh.sh_type, SHType::SYMTAB))
            .flat_map(|sh| {
                let strtab = &self.section_headers()[sh.sh_link as usize];
                let symbols = unsafe {
                    core::slice::from_raw_parts_mut(
                        self.start_address().offset(sh.sh_offset as isize) as *mut Symbol,
                        sh.sh_size / sh.sh_entsize,
                    )
                };
                symbols
                    .iter_mut()
                    /* SHN_UNDEF with name */
                    .filter(|s| s.st_shndx == 0 && s.st_name != 0)
                    .map(|s| {
                        let sym_name = unsafe {
                            crate::cstr2ruststr(
                                self.start_address()
                                    .offset(strtab.sh_offset as isize)
                                    .offset(s.st_name as isize),
                            )
                        };
                        println!("SHN_UNDEF: name={} {:#x}", sym_name, s.st_value);
                        sym_name
                    })
            })
            .collect()
    }

    pub fn get_all_symbol_names(&self) -> Vec<&'static str> {
        self.section_headers()
            .iter()
            .filter(|&sh| matches!(sh.sh_type, SHType::SYMTAB))
            .flat_map(|sh| {
                let strtab = &self.section_headers()[sh.sh_link as usize];
                let symbols = unsafe {
                    core::slice::from_raw_parts_mut(
                        self.start_address().offset(sh.sh_offset as isize) as *mut Symbol,
                        sh.sh_size / sh.sh_entsize,
                    )
                };
                symbols
                    .iter_mut()
                    /* SHN_UNDEF with name */
                    .filter(|s| s.st_shndx != 0 && s.st_name != 0)
                    .map(|s| {
                        let sym_name = unsafe {
                            crate::cstr2ruststr(
                                self.start_address()
                                    .offset(strtab.sh_offset as isize)
                                    .offset(s.st_name as isize),
                            )
                        };
                        sym_name
                    })
            })
            .collect()
    }

    pub fn calculate_needed_size(&self) -> ((usize, usize), (usize, usize)) {
        /* get needed size for allocation */
        self.section_headers()
            .iter()
            /* section has alloc flag need to be load */
            .filter(|&sh| sh.sh_flags & (SHFlags::ALLOC as usize) != 0)
            /* calculate the required size and align */
            .fold(((0, 0), (0, 0)), |td, sh| {
                // println!("{:?}", tdalign);
                fn alignup(v: usize, a: usize) -> usize {
                    (v as *const u8).align_offset(a) + v
                }

                let sec_align = sh.sh_addralign;
                let sec_size = alignup(sh.sh_size, 4);
                match sh.sh_flags & (SHFlags::WRITE as usize) {
                    /* text/rodata section has not write flag */
                    0 => (
                        (
                            alignup(td.0 .0, sec_align) + sec_size,
                            td.0 .1.max(sec_align),
                        ),
                        td.1,
                    ),
                    /* data/bss section has write flag */
                    _ => (
                        td.0,
                        (
                            alignup(td.1 .0, sec_align) + sec_size,
                            td.1 .1.max(sec_align),
                        ),
                    ),
                }
            })
    }

    pub const fn elf_header(&self) -> &EHeader {
        /* it's safe because we succeed to parse it */
        unsafe { &*self.ehaddr }
    }

    pub const fn section_headers(&self) -> &[SHeader] {
        /* it's safe because we succeed to parse it */
        unsafe { &*self.shaddr }
    }

    pub const fn start_address(&self) -> *const u8 {
        self.ehaddr as *const u8
    }

    pub unsafe fn relocateadd(rela: &Rela, sym: &Symbol, addr: *mut u8) {
        use RelaType::*;
        let rtype = &rela.rela_type();
        println!(
            "{:?} @{:p} [{:08x}] to sym@{:p} [{:#x}]",
            rtype,
            addr,
            (addr as *mut u32).read_unaligned(),
            sym,
            sym.st_value,
        );
        match rtype {
            RELAX => {}
            RISCV_32 | RISCV_64 => {
                (addr as *mut u32)
                    .write_unaligned((sym.symbol_value() as isize + rela.rela_addend()) as u32);
            }
            PCREL_LO12_I | PCREL_LO12_S => {
                /* NOTE: imm value for mv has been adjusted in previous HI20 */
            }
            PCREL_HI20 | CALL | CALL_PLT => {
                let offset = sym.symbol_value() as isize - addr as isize;
                let hi20 = ((offset + 0x800) as usize & 0xfffff000) as isize;
                let lo12 = (offset - hi20) & 0xfff;
                /* Adjust auipc (add upper immediate to pc) : 20bit */
                (addr as *mut u32)
                    .write_unaligned(((addr as *mut u32).read_unaligned() & 0xfff) | (hi20 as u32));
                /* Adjust remain 12bit */
                match (addr as *mut u32).offset(1).read_unaligned() & 0x7f {
                    // OPCODE_SW       0x23
                    // OPCODE_LUI      0x37
                    // RVI_OPCODE_MASK 0x7F
                    0x23 => {
                        /* Adjust imm for SW : S-type */
                        let imm11_5 = ((lo12 & 0xfe0) << (31 - 11)) as u32;
                        let imm4_0 = ((lo12 & 0x1f) << (11 - 4)) as u32;
                        (addr as *mut u32).offset(1).write_unaligned(
                            ((addr as *mut u32).offset(1).read_unaligned() & 0x1fff07f)
                                | imm11_5
                                | imm4_0,
                        );
                    }
                    _ => {
                        /* Adjust imm for MV(ADDI)/JALR : I-type */
                        (addr as *mut u32).offset(1).write_unaligned(
                            ((addr as *mut u32).offset(1).read_unaligned() & 0xfffff)
                                | ((lo12 << 20) as u32),
                        );
                    }
                }
            }
            BRANCH => {
                let offset = sym.symbol_value() as isize - addr as isize;
                /* P.23 Conditinal Branches : B type (imm=12bit) 0xfe000f80 */
                let val = (addr as *mut u32).read_unaligned() & 0xfe000f80;
                let imm12 = (offset as u32 & 0x1000) << (31 - 12);
                let imm11 = (offset as u32 & 0x800) >> (11 - 7);
                let imm10_5 = (offset as u32 & 0x7e0) << (30 - 10);
                let imm4_1 = (offset as u32 & 0x1e) << (11 - 4);

                assert_eq!(val, imm12 | imm11 | imm10_5 | imm4_1);

                println!(
                    "offset for Bx={} ({:#x}) (val={:#x})already set!",
                    offset, offset, val
                );
            }
            HI20 => {
                let hi20 = (sym.symbol_value() + 0x800 & 0xfffff000) as isize;
                (addr as *mut u32)
                    .write_unaligned(((addr as *mut u32).read_unaligned() & 0xfff) | (hi20 as u32));
            }
            LO12_I => {
                let hi20 = (sym.symbol_value() + 0x800 & 0xfffff000) as isize;
                let lo12 = (sym.symbol_value() as isize - hi20) & 0xfff;
                (addr as *mut u32).write_unaligned(
                    ((addr as *mut u32).read_unaligned() & 0xfff) | ((lo12 << 20) as u32),
                );
            }
            LO12_S => {
                let hi20 = (sym.symbol_value() + 0x800 & 0xfffff000) as isize;
                let lo12 = (sym.symbol_value() as isize - hi20) & 0xfff;
                /* Adjust imm for SW : S-type */
                let imm11_5 = ((lo12 & 0xfe0) << (31 - 11)) as u32;
                let imm4_0 = ((lo12 & 0x1f) << (11 - 4)) as u32;
                (addr as *mut u32).write_unaligned(
                    ((addr as *mut u32).read_unaligned() & 0x1fff07f) | imm11_5 | imm4_0,
                );
            }
            RVC_JUMP => {
                let offset = sym.symbol_value() as isize - addr as isize;
                /* P.111 Table 16.6 : Instruction listings for RVC 0x1ffc */
                let val = (addr as *mut u16).read_unaligned() & 0x1ffc;
                let imm11 = (offset as u16 & 0x800) << (12 - 11);
                let imm10 = (offset as u16 & 0x400) >> (10 - 8);
                let imm9_8 = (offset as u16 & 0x300) << (12 - 11);
                let imm7 = (offset as u16 & 0x80) >> (7 - 6);
                let imm6 = (offset as u16 & 0x40) << (12 - 11);
                let imm5 = (offset as u16 & 0x20) >> (5 - 2);
                let imm4 = (offset as u16 & 0x10) << (12 - 5);
                let imm3_1 = (offset as u16 & 0xe) << (12 - 10);

                assert_eq!(
                    val,
                    imm11 | imm10 | imm9_8 | imm7 | imm6 | imm5 | imm4 | imm3_1
                );

                println!(
                    "offset for C.J={} ({:#x}) (val={:#x})already set!",
                    offset, offset, val
                );
            }
            RVC_BRANCH => {
                let offset = sym.symbol_value() as isize - addr as isize;
                /* P.111 Table 16.6 : Instruction listings for RVC 0x1c7c */
                let val = (addr as *mut u16).read_unaligned() & 0x1c7c;
                let imm8 = (offset as u16 & 0x100) << (12 - 8);
                let imm7_6 = (offset as u16 & 0xc0) >> (6 - 5);
                let imm5 = (offset as u16 & 0x20) >> (5 - 2);
                let imm4_3 = (offset as u16 & 0x18) << (12 - 5);
                let imm2_1 = (offset as u16 & 0x6) << (12 - 10);

                assert_eq!(val, imm8 | imm7_6 | imm5 | imm4_3 | imm2_1);

                println!(
                    "offset for C.Bx={} ({:#x}) (val={:#x})already set!",
                    offset, offset, val
                );
            }
        }
    }
}
