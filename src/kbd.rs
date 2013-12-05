use utils::{outb, inb};
use core::slice;
use core::container::Container;
use stdio;

static NUMS: &'static [u8] = bytes!("01234567890");

static mut pos: u8 = 0;
#[no_mangle]
pub unsafe fn _interrupt_handler_kbd() {
    let keycode = inb(0x60);
    if (keycode == 2 || keycode == 3) {
        stdio::putc(NUMS[2]); // should be '2'. It is
        stdio::putc(65 + keycode); // should be 'C' (keycode = 2). It is. 
        stdio::putc(NUMS[keycode]); // should be '2', BUT IT ISN'T
        stdio::putc(124);
    }
    outb(0x20, 0x20);
}
