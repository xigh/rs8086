CPU     8086
BITS    16
ORG     0       ; !!! hey: in fact, we start at 0xf000:0x0000
%include "./expect.inc"

_start:
        CALL    test1
        HLT

test1:
        MOV     AX, 0x0105
        AAA
        RET

        ; must come after HLT or RET
        EXPECT  AX, 0x0005
        ; todo: flags
