use utils::{outb, inb};
use stdio;


static mut pos: u8 = 0;
#[no_mangle]
pub unsafe fn _interrupt_handler_kbd() {
    pos += 1;
    outb(0x20, 0x20);
    let key = inb(0x60);
    stdio::putchar(5 + (pos as u16), 6, key);
}
