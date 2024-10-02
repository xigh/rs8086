CPU     8086
BITS    16
ORG     0
%include "expect.inc"

_start:
        MOV     DX, 0
        MOV     AX, 0xC000
        CWD
        HLT

        EXPECT  __FILE__, __LINE__, AX, 0xC000
        EXPECT  __FILE__, __LINE__, DX, 0xFFFF
