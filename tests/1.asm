cpu 8086
bits 16
org 0

_start:
        mov dl, 7
        mov al, dl
        mov ah, ch
        or ah, bh
        pop cx
        cmp al, cl
        jnz _start
        hlt
