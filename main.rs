#[allow(ctypes)];
#[no_std];
#[feature(asm)];
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
    let mut i = 0;
    while true {
        i += 1;
    }
}

