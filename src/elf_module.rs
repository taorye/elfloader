use alloc::alloc::{alloc_zeroed, dealloc, Layout};
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::collections::LinkedList;
use alloc::rc;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::intrinsics;
use core::ptr;
use core::slice;

use crate::elf::headers::{SHFlags, SHType};
use crate::elf::section::{Rela, STBind, Symbol};
use crate::elf::ELFFile;

pub struct ElfModuleRoot {
    pub modules: LinkedList<rc::Rc<RefCell<ElfModule>>>,
}

impl ElfModuleRoot {
    pub fn load_elf_file(&mut self, elf_file: &ELFFile) -> Option<*const ElfModule> {
        let und_sym_names = elf_file.get_undefined_symbol_names();
        /* try find undefined global symbols */
        let und_syms = und_sym_names
            .iter()
            .map(|&name| {
                let sym_and_weak = self
                    .find_symbol_and_weak(name)
                    .unwrap_or((ptr::null(), rc::Weak::default()));
                (name, sym_and_weak)
            })
            .collect::<Vec<_>>();

        /* return None while can't find undefined */
        if und_syms.iter().any(|us| us.1 .0.is_null()) {
            println!("[err]undefined symbol can't be resolved");
            return None;
        }

        if elf_file
            .get_all_symbol_names()
            .iter()
            .any(|name| self.find_symbol(name).is_some())
        {
            println!("[err]global symbol has conflict");
            return None;
        }

        let em = ElfModule::new()
            /* allocate memory for text and data */
            .alloc_memory_with(&elf_file)
            /* fill undefined global symbols */
            .fill_undefined_symbols(und_syms)
            /* load section data into memory */
            .load_into_memory(&elf_file)
            /* update symbol value */
            .update_symbol_value_with(&elf_file)
            /* relocate text and data */
            .relocate_symbols_with(&elf_file);

        self.modules.push_back(rc::Rc::new(RefCell::new(em)));
        self.modules
            .back()
            .and_then(|rcmod| Some(rcmod.as_ptr() as *const ElfModule))
    }

    pub fn unload_elf_module(&mut self, elf_module: *const ElfModule) {
        self.modules
            .iter()
            .enumerate()
            .rfind(|&(_, m)| m.as_ptr() as usize == elf_module as usize)
            .and_then(|(idx, em)| {
                em.borrow().dependents.is_empty().then(|| {
                    em.borrow()
                        .dependencies
                        .iter()
                        .map(|weakpm| {
                            weakpm.upgrade().and_then(|pm| {
                                Some(
                                    pm.borrow_mut()
                                        .dependents
                                        .retain(|m| m.as_ptr() as usize != elf_module as usize),
                                )
                            })
                        })
                        .count();
                    idx
                })
            })
            .and_then(|idx| Some(self.modules.remove(idx)));
    }

    pub fn find_symbol(&self, name: &str) -> Option<*const u8> {
        self.modules
            .iter()
            .find_map(|m| m.borrow().find_symbol(name))
    }

    fn find_symbol_and_weak(
        &self,
        name: &str,
    ) -> Option<(*const u8, rc::Weak<RefCell<ElfModule>>)> {
        self.modules.iter().find_map(|m| {
            m.borrow()
                .find_symbol(name)
                .and_then(|symaddr| Some((symaddr, rc::Rc::downgrade(m))))
        })
    }
}

pub struct ElfModule {
    pub dependents: Vec<rc::Rc<RefCell<Self>>>,
    pub dependencies: Vec<rc::Weak<RefCell<Self>>>,
    pub text_info: Option<(*mut u8, Layout)>,
    pub data_info: Option<(*mut u8, Layout)>,
    pub symbol_info: BTreeMap<&'static str, *const u8>,
}

impl Drop for ElfModule {
    fn drop(&mut self) {
        // free memory
        // `str` in symbol_info
        // `[u8]` in text_info & data_info
        self.symbol_info
            .iter()
            .map(|(&n, _)| unsafe {
                Box::from_raw(ptr::addr_of!(*n) as *mut str);
            })
            .count();
        self.text_info
            .and_then(|(p, l)| unsafe { Some(dealloc(p, l)) });
        self.data_info
            .and_then(|(p, l)| unsafe { Some(dealloc(p, l)) });
    }
}

impl ElfModule {
    pub fn new() -> Self {
        /* initialize module with nothing */
        Self {
            dependents: Vec::new(),
            dependencies: Vec::new(),
            text_info: None,
            data_info: None,
            symbol_info: BTreeMap::new(),
        }
    }

    pub fn alloc_memory_with(self, elf_file: &ELFFile) -> Self {
        let (text, data) = {
            /* get needed size for allocation */
            let ((text_size, text_align), (data_size, data_align)) =
                elf_file.calculate_needed_size();
            /* allocate memory to load text and data */
            fn get_layout_and_alloc_memory(s: usize, a: usize) -> Option<(*mut u8, Layout)> {
                Layout::from_size_align(s, a)
                    .ok()
                    .filter(|l| l.size() > 0)
                    .and_then(|l| {
                        let p = unsafe { alloc_zeroed(l) };
                        Some((p, l))
                    })
            }
            // fn dealloc_memory_by_layout(p: *mut u8, l: Layout) {
            //     unsafe { dealloc(p, l) }
            // }
            (
                get_layout_and_alloc_memory(text_size, text_align),
                get_layout_and_alloc_memory(data_size, data_align),
            )
        };

        let mut em = self;
        text.and_then(|t| {
            em.text_info.replace(t);
            Some(println!(
                "[success]allocate text@{:p} with {}bytes",
                t.0,
                t.1.size()
            ))
        });
        data.and_then(|d| {
            em.data_info.replace(d);
            Some(println!(
                "[success]allocate data@{:p} with {}bytes",
                d.0,
                d.1.size()
            ))
        });
        em.print_text_and_data();
        em
    }

    pub fn fill_undefined_symbols(
        self,
        symbols: impl IntoIterator<Item = (&'static str, (*const u8, rc::Weak<RefCell<ElfModule>>))>,
    ) -> Self {
        let mut em = self;
        symbols
            .into_iter()
            .map(|(symname, (symvalue, pm))| {
                em.symbol_info
                    .entry(Box::leak(symname.to_string().into_boxed_str()))
                    .or_insert(symvalue);
                em.dependencies.push(pm)
            })
            .count();
        em
    }

    pub fn load_into_memory(self, elf_file: &ELFFile) -> Self {
        /* load section data into memory */
        println!("[trying]Load section data into memory");
        let shstrtab = &elf_file.section_headers()[elf_file.elf_header().e_shstrndx as usize];
        elf_file
            .section_headers()
            .iter()
            /* section has alloc flag need to be load */
            .filter(|&sh| sh.sh_flags & (SHFlags::ALLOC as usize) != 0)
            .fold((0, 0), |tdoff, sh| {
                fn alignup(v: usize, a: usize) -> usize {
                    (v as *const u8).align_offset(a) + v
                }

                let sec_align = sh.sh_addralign;
                let sec_size = alignup(sh.sh_size, 4);

                let (cur_tdoff, (baseaddr, off)) = match sh.sh_flags & (SHFlags::WRITE as usize) {
                    // text/rodata section has not write flag
                    0 => (
                        (alignup(tdoff.0, sec_align) + sec_size, tdoff.1),
                        (self.text_info, alignup(tdoff.0, sec_align)),
                    ),
                    // data|bss section has write flag
                    _ => (
                        (tdoff.0, alignup(tdoff.1, sec_align) + sec_size),
                        (self.data_info, alignup(tdoff.1, sec_align)),
                    ),
                };
                /* copy datas to memory */
                baseaddr.and_then(|(base, _)| unsafe {
                    let secaddr = base.offset(off as isize);
                    if let SHType::NOBITS = sh.sh_type {
                        secaddr.write_bytes(0, sh.sh_size)
                    } else {
                        intrinsics::copy_nonoverlapping(
                            elf_file.start_address().offset(sh.sh_offset as isize),
                            secaddr,
                            sh.sh_size,
                        );
                    }
                    // update section address to system memory
                    sh.section_address_set(secaddr as usize);

                    Some(println!(
                        "{}. offset {:#x} -> {:#x}, size: {}",
                        crate::cstr2ruststr(
                            elf_file
                                .start_address()
                                .offset(shstrtab.sh_offset as isize)
                                .offset(sh.sh_name as isize)
                        ),
                        sh.sh_offset,
                        sh.sh_addr,
                        sh.sh_size
                    ))
                });
                cur_tdoff
            });
        println!("[success]Load section data into memory");
        self.print_text_and_data();
        self
    }

    pub fn update_symbol_value_with(self, elf_file: &ELFFile) -> Self {
        /* update symbol value */
        println!("[trying]update symbol value");
        elf_file
            .section_headers()
            .iter()
            .filter(|&sh| matches!(sh.section_type(), SHType::SYMTAB))
            .flat_map(|sh| {
                let strtab = &elf_file.section_headers()[sh.sh_link as usize];
                let symbols = unsafe {
                    slice::from_raw_parts_mut(
                        elf_file.start_address().offset(sh.sh_offset as isize) as *mut Symbol,
                        sh.sh_size / sh.sh_entsize,
                    )
                };
                symbols.iter_mut().filter(|s| s.st_name != 0).map(|s| {
                    let symname = unsafe {
                        crate::cstr2ruststr(
                            elf_file
                                .start_address()
                                .offset(strtab.sh_offset as isize)
                                .offset(s.st_name as isize),
                        )
                    };
                    // SHN_UNDEF 	    0
                    // SHN_LORESERVE    0xff00
                    // SHN_LOPROC 	    0xff00
                    // SHN_HIPROC 	    0xff1f
                    // SHN_LOOS 	    0xff20
                    // SHN_HIOS 	    0xff3f
                    // SHN_ABS 	        0xfff1
                    // SHN_COMMON 	    0xfff2
                    // SHN_XINDEX 	    0xffff
                    // SHN_HIRESERVE 	0xffff
                    match s.st_shndx {
                        0xfff2 => {
                            panic!("Re-compile with -fno-common");
                        }
                        0x0 => {
                            print!("SHN_UNDEF: name={} {:#x}", symname, s.st_value);
                            self.find_symbol(symname).and_then(|exsym| {
                                s.symbol_value_set(exsym as usize);
                                Some(print!("->{:#x}", s.st_value))
                            });
                            println!()
                        }
                        secidx => {
                            if secidx == 0xfff1 {
                                println!("SHN_ABS: st_value={:#x}", s.st_value)
                            } else {
                                let secbase = elf_file.section_headers()[secidx as usize].sh_addr;
                                println!(
                                    "Other: {:#x}+{:#x}={:#x}",
                                    s.st_value,
                                    secbase,
                                    s.st_value + secbase
                                );
                                s.symbol_value_set(s.st_value + secbase);
                            }
                            if let STBind::GLOBAL = s.symbol_bind() {
                                self.add_symbol(
                                    Box::leak(symname.to_string().into_boxed_str()),
                                    s.st_value,
                                );
                            }
                        }
                    }
                    println!("    {:?}", s);
                })
            })
            .count();
        println!("[success]update symbol value");
        self
    }

    pub fn relocate_symbols_with(self, elf_file: &ELFFile) -> Self {
        /* relocate text and data */
        println!("[trying]relocate text and data");
        elf_file
            .section_headers()
            .iter()
            .filter(|&sh| matches!(sh.section_type(), SHType::RELA))
            .map(|relasec| {
                let symsec = &elf_file.section_headers()[relasec.sh_link as usize];
                let dstsec = &elf_file.section_headers()[relasec.sh_info as usize];
                let dstsecbase = dstsec.sh_addr;
                let (relas, symbols) = unsafe {
                    (
                        slice::from_raw_parts(
                            elf_file.start_address().offset(relasec.sh_offset as isize)
                                as *const Rela,
                            relasec.sh_size / relasec.sh_entsize,
                        ),
                        slice::from_raw_parts(
                            elf_file.start_address().offset(symsec.sh_offset as isize)
                                as *const Symbol,
                            symsec.sh_size / symsec.sh_entsize,
                        ),
                    )
                };

                relas
                    .iter()
                    .map(|r| {
                        let sym = &symbols[r.symbol_offset()];
                        let addr = dstsecbase + r.r_offset;
                        unsafe {
                            // real relocate
                            ELFFile::relocateadd(r, sym, addr as *mut u8);
                        }
                    })
                    .count();
            })
            .count();
        println!("[success]relocate text and data");
        self
    }

    fn add_symbol(&self, name: &'static str, sym: usize) {
        unsafe { &mut *(ptr::addr_of!(*self) as *mut Self) }
            .symbol_info
            .entry(name)
            .or_insert(sym as *const u8);
    }

    pub fn find_symbol(&self, name: &str) -> Option<*const u8> {
        self.symbol_info.get(&name).and_then(|p| Some(*p))
    }

    fn print_text_and_data(&self) {
        fn _print_info(p: *mut u8, s: usize) {
            let s = unsafe { core::slice::from_raw_parts(p, s) };
            crate::hex_dump(s)
        }
        self.text_info
            .and_then(|(p, l)| {
                println!("text@{:p} with {}bytes", p, l.size());
                Some(_print_info(p, l.size()))
            })
            .unwrap_or_else(|| println!("this module has no text"));
        self.data_info
            .and_then(|(p, l)| {
                println!("data@{:p} with {}bytes", p, l.size());
                Some(_print_info(p, l.size()))
            })
            .unwrap_or_else(|| println!("this module has no data"));
    }
}
