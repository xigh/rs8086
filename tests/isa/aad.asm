CPU     8086
BITS    16
ORG     0       ; !!! hey: in fact, we start at 0xf000:0x0000

_start:
        MOV     AX, 0x0105
        AAD
        HLT

%include "expect.inc"

        ; must come after HLT
        EXPECT  AX, 0x000F
        EXPECT  CF, 1
