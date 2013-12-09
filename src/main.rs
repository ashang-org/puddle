#[allow(ctypes)];
#[no_std];
#[feature(asm)];
#[feature(macro_rules)];

#[path = "rust-core/core/mod.rs"]
mod core;
pub mod idt;
pub mod kbd;
pub mod mem;
pub mod pic;
pub mod serial;
pub mod stdio;
pub mod utils;

#[no_mangle]
pub unsafe fn main() {
    serial::init();
    idt::idt_install();
    stdio::clear_screen();
    serial::write("Starting...\n");
}

