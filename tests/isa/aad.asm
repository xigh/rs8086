ORG 0
BITS 16

_start:
    MOV AX, 0x0105
    AAD
    RET

; EXPECT AX == 15 (0x000F)
