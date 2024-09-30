CPU     8086
BITS    16
ORG     0
%include "expect.inc"

_start:
        NOP
        HLT

        EXPECT  __FILE__, __LINE__, AX, 0
