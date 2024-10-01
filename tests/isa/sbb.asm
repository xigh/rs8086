CPU     8086
BITS    16
ORG     0       ; !!! hey: in fact, we start at 0xf000:0x0000
%include "expect.inc"

_start:
        MOV AL, 0x05
        STC
        SBB AL, 0x01
        HLT

        EXPECT  __FILE__, __LINE__, AL, 0x03
