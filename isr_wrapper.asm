use32
global  _isr_wrapper
global  idt_load
align   4

extern _interrupt_handler
extern idtp

_isr_wrapper:
    pushad
    call    _interrupt_handler
    popad
    iret

idt_load:
    lidt [eax]
    ret
