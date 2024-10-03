CPU     8086
BITS    16
ORG     0
%include "expect.inc"

_start:
        MOV     AX, 0x15
        AAS
        HLT

        EXPECT  __FILE__, __LINE__, AX, 0x0005
        EXPECT  __FILE__, __LINE__, DONE
