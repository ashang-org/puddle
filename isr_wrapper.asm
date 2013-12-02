use32
global  _interrupt_handler_kbd_wrapper
global  idt_load
align   4

extern _interrupt_handler_kbd

_interrupt_handler_kbd_wrapper: 
    pushad
    call    _interrupt_handler_kbd
    popad
    iret

