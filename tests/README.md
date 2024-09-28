# tests

## error message

The test framework can now display an explicit error message when an error occurs.

In file `tests/error.asm`:

```asm
CPU     8086
BITS    16
ORG     0
%include "./expect.inc"

_start:
        HLT

        EXPECT  __FILE__, __LINE__, AX, 1 ; this is an error as by default AX is initialized to 0x0000
```

Now, compile and run the test in the emulator:

```bash
$ nasm -f bin -DDEBUG -Itests -otests/error.bin tests/error.asm 
$ cargo r --bin emu8086 -- -test -hide-header -dump-regs-on-halt tests/error.bin 
```

this will output:

```text
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/emu8086 -test -hide-header -dump-regs-on-halt tests/error.bin`
0F0000 f4               hlt
tests/error.bin: tests/error.asm:9: AX: got 0x0000, expected 0x0001
```

Here you can see that the error message is displayed and the program stops, showing where the error is (line 9 in `tests/error.asm`).

## how to test `aaa` instruction

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

## how to simply compile `aaa` instruction

```bash
$ nasm -f bin -Itests -otests/isa/aaa.bin tests/isa/aaa.asm 
$ ndisasm tests/isa/aaa.bin 
```

```text
00000000  E80100            call 0x4
00000003  F4                hlt
00000004  B80501            mov ax,0x105
00000007  37                aaa
00000008  C3                ret
```
