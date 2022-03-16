use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\x1b[93mPanic!\x1b[0m");
    if let Some(p) = info.location() {
        println!("{:?}: {}", p, info.message().unwrap());
    } else {
        println!("no information");
    }

    loop {
        unsafe {
            core::arch::riscv32::wfi();
        }
    }
}
