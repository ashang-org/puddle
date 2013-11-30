use32
global   _isr_wrapper
align   4

extern _interrupt_handler

_isr_wrapper:
    pushad
    call    _interrupt_handler
    popad
    iret
