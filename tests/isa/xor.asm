CPU     8086
BITS    16
ORG     0
%include "expect.inc"

_start:
        MOV     AL, 0x05
        XOR     AL, 0x01
        HLT

        EXPECT  __FILE__, __LINE__, AX, 0x04
        EXPECT  __FILE__, __LINE__, DONE
