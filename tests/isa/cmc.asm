CPU     8086
BITS    16
ORG     0
%include "expect.inc"

_start:
        CLC
        CMC
        HLT

        EXPECT  __FILE__, __LINE__, CF, 1
        EXPECT  __FILE__, __LINE__, DONE
