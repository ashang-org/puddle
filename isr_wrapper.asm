use32
global  _isr_wrapper
global  _idt_load
align   4

extern _interrupt_handler

_isr_wrapper:
    pushad
    call    _interrupt_handler
    popad
    iret
