; nasm -f bin -o tests/1.bin tests/1.asm

CPU     8086
BITS    16
ORG     0       ; !!! hey: in fact, we start at 0xf000:0x0000

_start:
        ; this is pure junk sequence to test the CPU
        MOV     DL, 7
        MOV     AL, DL
        MOV     AH, CH
        OR      AH, BH
        POP     CX
        CMP     AL, CL
        JNZ     _start
        HLT
