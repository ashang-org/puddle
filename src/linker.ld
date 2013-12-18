ENTRY(start)
OUTPUT_FORMAT("elf32-i386")
OUTPUT_ARCH(i386)
SECTIONS {
  . = 0x7e00;
  .text : {
    *(.text)
  }

  .bss : {
    *(.bss*);
  }

  . = 0x10000;
  .donothing : {
    *(._binary_build_programs_do_nothing_start);
  }
 }
