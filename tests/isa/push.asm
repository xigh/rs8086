CPU     8086
BITS    16
ORG     0       ; !!! hey: in fact, we start at 0xf000:0x0000
%include "expect.inc"

_start:
        MOV     SP, 0xfffe
        MOV     AX, 0
        MOV     SS, AX

        MOV     BX, 1
        PUSH    BX
        POP     AX
        HLT

        EXPECT  __FILE__, __LINE__, AX, 1
        EXPECT  __FILE__, __LINE__, DONE
