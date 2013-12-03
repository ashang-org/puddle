use utils::{inb, outb, io_wait};

static PIC1		:u16 = 0x20;		/* IO base address for master PIC */
static PIC2		:u16 = 0xA0;		/* IO base address for slave PIC */
static PIC1_COMMAND	:u16 = PIC1;
static PIC1_DATA	:u16 = PIC1 + 1;
static PIC2_COMMAND	:u16 = PIC2;
static PIC2_DATA	:u16 = PIC2 + 1;

static ICW1_ICW4	:u8 = 0x01;		/* ICW4 (not) needed */
static ICW1_SINGLE	:u8 = 0x02;		/* Single (cascade) mode */
static ICW1_INTERVAL4	:u8 = 0x04;		/* Call address interval 4 (8) */
static ICW1_LEVEL	:u8 = 0x08;		/* Level triggered (edge) mode */
static ICW1_INIT	:u8 = 0x10;		/* Initialization - required! */

static ICW4_8086	:u8 = 0x01;		/* 8086/88 (MCS-80/85) mode */
static ICW4_AUTO	:u8 = 0x02;		/* Auto (normal) EOI */
static ICW4_BUF_SLAVE	:u8 = 0x08;		/* Buffered mode/slave */
static ICW4_BUF_MASTER	:u8 = 0x0C;		/* Buffered mode/master */
static ICW4_SFNM	:u8 = 0x10;		/* Special fully nested (not) */


/*
arguments:
	offset1 - vector offset for master PIC
		vectors on the master become offset1..offset1+7
	offset2 - same for slave PIC: offset2..offset2+7
*/
pub unsafe fn PIC_remap(offset1: u8, offset2: u8)
{
    
    let a1 = inb(PIC1_DATA);           // save masks
    let a2 = inb(PIC2_DATA);           // save masks
    
    outb(PIC1_COMMAND, ICW1_INIT+ICW1_ICW4);  // starts the initialization sequence (in cascade mode)
    io_wait();
    outb(PIC2_COMMAND, ICW1_INIT+ICW1_ICW4);
    io_wait();
    outb(PIC1_DATA, offset1);                 // ICW2: Master PIC vector offset
    io_wait();
    outb(PIC2_DATA, offset2);                 // ICW2: Slave PIC vector offset
    io_wait();
    outb(PIC1_DATA, 4);                       // ICW3: tell Master PIC that there is a slave PIC at IRQ2 (0000 0100)
    io_wait();
    outb(PIC2_DATA, 2);                       // ICW3: tell Slave PIC its cascade identity (0000 0010)
    io_wait();
    
    outb(PIC1_DATA, ICW4_8086);
    io_wait();
    outb(PIC2_DATA, ICW4_8086);
    io_wait();
    
    outb(PIC1_DATA, a1);   // restore saved masks.
    outb(PIC2_DATA, a2);
}
