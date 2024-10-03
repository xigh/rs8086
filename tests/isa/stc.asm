CPU     8086
BITS    16
ORG     0       ; !!! hey: in fact, we start at 0xf000:0x0000
%include "./expect.inc"

_start:
        STC
        HLT

        EXPECT  __FILE__, __LINE__, CF, 1
        EXPECT  __FILE__, __LINE__, DONE
