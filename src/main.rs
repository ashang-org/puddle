#[allow(ctypes)];
#[no_std];
#[feature(asm)];
#[feature(macro_rules)];

extern mod core;
pub mod idt;
pub mod kbd;
pub mod mem;
pub mod pic;
pub mod elf;
pub mod serial;
pub mod stdio;
pub mod utils;
pub mod exec;

#[no_mangle]
pub unsafe fn main() {
    stdio::clear_screen();
    serial::init();
    idt::idt_install();
    serial::write("Starting...\n");
    exec::exec(0xd330);
}

