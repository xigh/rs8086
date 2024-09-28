# tests

```bash
nasm -f bin -DDEBUG -Itests -otests/isa/aaa.bin tests/isa/aaa.asm 
ndisasm tests/isa/aaa.bin 
```

```text
00000000  E80100            call 0x4
00000003  F4                hlt
00000004  B80501            mov ax,0x105
00000007  37                aaa
00000008  C3                ret
00000009  41                inc cx         -> "A" in ascii
0000000A  58                pop ax         -> "X" in ascii
0000000B  05                db 0x05
0000000C  00                db 0x00
```

```bash
nasm -f bin -Itests -otests/isa/aaa.bin tests/isa/aaa.asm 
ndisasm tests/isa/aaa.bin 
```

```text
00000000  E80100            call 0x4
00000003  F4                hlt
00000004  B80501            mov ax,0x105
00000007  37                aaa
00000008  C3                ret
```
