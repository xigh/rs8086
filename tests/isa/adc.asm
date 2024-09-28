CPU     8086
BITS    16
ORG     0
%include "expect.inc"

_start:
        MOV     AL, 0x05
        STC
        ADC     AL, 0x01
        HLT

        EXPECT  __FILE__, __LINE__, AL, 0x07
