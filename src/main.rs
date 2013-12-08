#[allow(ctypes)];
#[no_std];
#[feature(asm)];
//#[no_core];
#[feature(macro_rules)];


#[path = "rust-core/core/mod.rs"]
mod core;
pub mod stdio;
pub mod kbd;
pub mod idt;
pub mod utils;
pub mod pic;
extern {
    fn run_interrupt ();
    fn test1 ();
}

#[no_mangle]
pub unsafe fn main() {
    idt::idt_install();
    stdio::clear_screen(stdio::Green);
    test1();
    run_interrupt();
}
