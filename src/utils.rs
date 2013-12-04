use stdio;
pub unsafe fn outb(port: u16, value: u8) {
    asm!("outb %al, %dx"
         :
         : "{dx}" (port), "{al}" (value)
         :
         : "volatile" );
}

pub unsafe fn inb(port: u16) -> u8 {
    let mut ret: u8;
    stdio::putc(66);
    asm!("inb %dx, %al"
         : "={al}"(ret)
         : "{dx}"(port)
         :
         : "volatile" );
    stdio::putc(65);
    stdio::putc(ret);
    return ret;
}

pub unsafe fn io_wait() {
    asm!("jmp 1f\n\t
          1:jmp 2f\n\t
          2:" );
}
