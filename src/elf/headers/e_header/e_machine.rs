#[repr(u16)]
#[derive(Debug)]
pub enum EMachine {
    NONE = 0,
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    RISCV = 243,
    //todo!();
}
