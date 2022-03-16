#![no_std]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(linked_list_remove)]
#![feature(ptr_const_cast)]
#![feature(core_intrinsics)]
#![feature(stdsimd)]
#![allow(dead_code)]

#[macro_use]
mod console;
mod allocator;
mod panic;

mod elf;
mod elf_module;

use elf::ELFFile;
use elf_module::ElfModule;
use elf_module::ElfModuleRoot;

extern crate alloc;

static mut ELF_MODULE_ROOT: ElfModuleRoot = ElfModuleRoot {
    modules: alloc::collections::LinkedList::new(),
};

extern "C" {
    // need to be impl which used in console.rs
    fn rust_console_putbytes(bs: *const u8, len: usize);
    // need to be impl which used in allocator.rs
    fn rust_aligned_alloc(alignment: usize, size: usize) -> *mut u8;
    fn rust_free(ptr: *mut u8);
}

use core::ptr;
use core::slice;
#[no_mangle]
pub unsafe extern "C" fn rust_elf_load(elf_buf: *const u8) -> *const ElfModule {
    ELFFile::parse(elf_buf)
        .ok()
        .and_then(|elf_file| ELF_MODULE_ROOT.load_elf_file(&elf_file))
        .unwrap_or(ptr::null())
}

/* ensure symbol_name is valid str */
#[no_mangle]
pub unsafe extern "C" fn rust_elf_sym(
    elf_module: *const ElfModule,
    symbol_name: *const u8,
) -> *const u8 {
    let symname = cstr2ruststr(symbol_name);
    elf_module
        .as_ref()
        .and_then(|em| em.find_symbol(symname))
        .or_else(|| ELF_MODULE_ROOT.find_symbol(symname))
        .unwrap_or(ptr::null())
}

#[no_mangle]
pub unsafe extern "C" fn rust_elf_unload(elf_module: *const ElfModule) {
    ELF_MODULE_ROOT.unload_elf_module(elf_module);
}

unsafe fn cstr2ruststr<'a>(s: *const u8) -> &'a str {
    let mut slen = 0usize;

    while *s.offset(slen as isize) != 0 {
        slen += 1;
    }
    let strbytes = slice::from_raw_parts(s, slen);
    core::str::from_utf8_unchecked(strbytes)
}

fn hex_dump(arr: &[u8]) {
    arr.iter()
        .fold(0, |idx, t| {
            match idx {
                0 => print!("{: >#x}0:\t", idx / 16),
                x if x % 16 == 0 => print!("\r\n{: >#x}0:\t", idx / 16),
                _ => print!(" "),
            }
            print!("{:02x}", t);
            idx + 1
        })
        .gt(&0)
        .then(|| println!());
}
