#[allow(ctypes)];
#[no_std];
//#[no_core];
#[feature(macro_rules)];


mod core;
pub mod stdio;
pub mod idt;


#[no_mangle]
pub unsafe fn main() {
    idt::idt_install();
    stdio::clear_screen(stdio::Green);
    stdio::write("Hello!aaa", 2, 3);
}

