CPU     8086
BITS    16
ORG     0       ; !!! hey: in fact, we start at 0xf000:0x0000

_start:
        MOV     AX, 0x0105
        AAD     ; aad 
        RET

; EXPECT AX == 15 (0x000F)
