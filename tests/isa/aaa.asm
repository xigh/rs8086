ORG 0
BITS 16

_start:
    MOV AX, 15
    AAA
    RET

; EXPECT AX == 0105h
