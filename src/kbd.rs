use utils::{outb, inb};
use core::slice;
use core::container::Container;
use stdio;

static SCAN_CODE_MAPPING: &'static [u8] = bytes!("\
\x00\x1B1234567890-=\x08\tqwertyuiop[]\n?asdfghjkl;'`?\\zxcvbnm,./?*? ?????????????789-456+1230.?????");
static SCAN_CODE_MAPPING_SHIFTED: &'static [u8] = bytes!("\
\x00\x1B!@#$%^&*()_+\x08\tQWERTYUIOP{}\n?ASDFGHJKL:\"~?|ZXCVBNM<>??*? ?????????????789-456+1230.?????");


static mut pos: u8 = 0;
#[no_mangle]
pub unsafe fn _interrupt_handler_kbd() {
    let keycode = inb(0x60);
    if (keycode < SCAN_CODE_MAPPING.len() as u8) {
        stdio::putc(SCAN_CODE_MAPPING[keycode]);
    }
    outb(0x20, 0x20);
}
