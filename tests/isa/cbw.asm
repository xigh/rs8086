CPU     8086
BITS    16
ORG     0
%include "expect.inc"

_start:
        MOV     AL, 0xC0
        CBW
        HLT

        EXPECT  __FILE__, __LINE__, AX, 0xFFC0
