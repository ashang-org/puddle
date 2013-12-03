#[allow(ctypes)];
#[no_std];
#[feature(asm)];
//#[no_core];
#[feature(macro_rules)];


mod core;
pub mod stdio;
pub mod idt;
extern {
    fn run_interrupt ();
    fn test1 ();
}

#[no_mangle]
pub unsafe fn main() {
    //idt::idt_install();
    stdio::clear_screen(stdio::Green);
    test1();
    run_interrupt();

    stdio::write(2, 3, "Hello! :)");
    let mut i: u32 = 0;
    let mut c: u8 = 65; // 'A'
    let N: u32 = 80000000; // big enough number so that it goes slowly
    loop {
        i += 1;
        if (i % N == 0) {
            c += 1;
            stdio::putchar(2, 4, c);
        }
    }
}

