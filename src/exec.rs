use core::slice::Slice;
use core::mem::transmute;
use elf;
use stdio;

unsafe fn jmp(addr: u32) {
    asm!("jmp *($0)"
         :
         : "r" (addr));
}

pub unsafe fn exec(addr: uint) {
    let slice = Slice {data: (addr as *u8), len: 100};
    let header = elf::read_header(transmute(slice));
    stdio::putc(header.e_ident.ei_mag[1]);
    stdio::putc(header.e_ident.ei_mag[2]);
    stdio::putc(header.e_ident.ei_mag[3]);
    stdio::putc(' ' as u8);
    stdio::print_num(header.e_entry as uint);
    //jmp(header.e_entry);
}
