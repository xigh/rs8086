CPU     8086
BITS    16
ORG     0
%include "expect.inc"

_start:
        CLI
        HLT

        EXPECT  __FILE__, __LINE__, IF, 0
