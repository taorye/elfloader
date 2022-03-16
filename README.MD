<p align = "center">
<b>Elfloader library</b>
<i>for the Rust programming language.</i>
</p>

-------

This library is going to help you load **.o** file into your system and make you easier to use the global functions and data in this ELF file.

## Compile
```sh
1. rustup override set nightly
2. cargo -Z build-std=core,alloc build [--release]
```
**OR**
```sh
cargo build [--release]
```

## Tip
We can just simply use `cargo build [--release]` because of `rust-toolchain` and `.cargo/config.toml`, then C-compatible static library `libelfloader.a` will be generated in directory `target/<arch>/[debug|release]` and I have also offered the C header file in `include` for detail.

## How it do

### Load
1. load `elf header` and `section headers` into memory for **Analyze**
2. get section `size` and `alignment` information from `section headers` for **Allocation**
3. calculate the required `size` and `align`, and then **Allocate** memories
4. load `.text,.data,.bss,.rodata` section into memory
5. load `symtab` through `section header` and **Resolve** all symbols' value
6. read `relatab` and then **Relocate** symbols which in target section
7. ***now the code can be executed normaly***

