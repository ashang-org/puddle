ENTRY(start)
OUTPUT_FORMAT("elf32-i386")
OUTPUT_ARCH(i386)
SECTIONS {
  . = 0x7e00;
  .text : {
    *(.text)
  }

  .data : {
    *(.data .rodata)
  }

  .bss : {
    *(.bss*);
  }
 }
