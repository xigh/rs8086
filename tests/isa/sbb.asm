CPU     8086
BITS    16
ORG     0       ; !!! hey: in fact, we start at 0xf000:0x0000

_start:
        MOV AL, 0x05
        STC
        SBB AL, 0x01
        RET

; EXPECT AL = 0x04, CF = 1

