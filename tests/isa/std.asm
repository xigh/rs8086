CPU     8086
BITS    16
ORG     0       ; !!! hey: in fact, we start at 0xf000:0x0000
%include "./expect.inc"

_start:
        STD
        HLT

        EXPECT  __FILE__, __LINE__, DF, 1
