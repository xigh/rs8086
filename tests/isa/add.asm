CPU     8086
BITS    16
ORG     0       ; !!! hey: in fact, we start at 0xf000:0x0000

_start:
        MOV AL, 0x05
        AND AL, 0x01
        HLT

; EXPECT AX == 0x01
