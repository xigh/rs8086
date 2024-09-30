# 80[_12]86 ISA

For more informations about x86 opcodes, you can read the following documents:

- [Intel® 64 and IA-32 Architectures Software Developer Manuals](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)
- [AMD64 Architecture Programmer's Manual Volume 2: System Programming](https://developer.amd.com/wp-content/resources/AMD64-Architecture-Programmer-s-Manual-Volume-2.pdf)
- [Wikipedia x86 opcode listing](https://en.wikipedia.org/wiki/X86_instruction_listings)
- [Shell-storm x86 opcode listing](https://shell-storm.org/x86doc/)
- [Linux kernel x86 opcode listing](https://github.com/torvalds/linux/blob/master/arch/x86/lib/x86-opcode-map.txt)
- [PCjs Intel 8086 opcodes](https://www.pcjs.org/documents/manuals/intel/8086/) [this one is very good]

and so many others... (i'll try to list them as i go)

Table of contents:

- [operands](#operands)
- [flags](#flags)
- [instructions](#instructions)
- [opcode map](#opcode-map)

## Operands

- GENERAL PURPOSE REGISTERS:  AH, AL, BL, BH, CH, CL, DH, DL,
        AX, BX, CX, DX, DI, SI, BP, SP.

- FLAGS: CF, PF, AF, ZF, SF, TF, IF, DF, OF.

- SEGMENT REGISTERS: DS, ES, SS, and only as second operand: CS.

- MEMORY ADDRESSING: [BX+SI], [BX+DI], [BP+SI], [BP+DI], [SI], [DI], [BP], [BX] (todo)

- IMMEDIATE VALUE: 8 or 16 bits : 0x12, 0x1234, etc.

## Flags

- CF - Carry flag

This flag is set if the last arithmetic operation generated a carry or a borrow.

- PF - Parity flag

This flag is set if the last arithmetic operation generated an even number of 1-bits in the result.

- AF - Auxiliary carry flag

This flag is set if the last arithmetic operation generated a carry or borrow from bit 3 to bit 4. Todo: play with BCD and this flag.

- ZF - Zero flag

This flag is set if the last arithmetic or logical operation generated a zero result.

- SF - Sign flag

This flag is set if the last arithmetic or logical operation generated a negative result.

- TF - Trap flag

This flags puts the processor in single step mode for debugging. In this mode, the processor will generate an interrupt after each instruction.

- IF - Interrupt flag

This flag enables or disables interrupts.

- DF - Direction flag

This flag is used in string instructions to determine the direction of the operation.

- OF - Overflow flag

This flag is set if the last arithmetic operation generated an overflow.

## Instructions

- [ ] [AAA](#aaa---ascii-adjust-after-addition) - ASCII adjust after addition
- [ ] [AAD](#aad---ascii-adjust-after-division) - ASCII adjust after division
- [ ] AAM - ASCII adjust after multiplication
- [ ] AAS - ASCII adjust after subtraction
- [ ] ADC - Add with carry
- [ ] ADD - Add
- [ ] AND - And
- [ ] CALL - Call
- [ ] CBW - Convert byte to word
- [X] CLC - Clear carry flag
- [X] CLD - Clear direction flag
- [X] CLI - Clear interrupt flag
- [ ] CMC - Complement carry flag
- [ ] CMP - Compare
- [ ] CMPS - Compare strings
    - [ ] CMPSB - Compare strings byte
    - [ ] CMPSW - Compare strings word
- [ ] CWD - Convert word to double word
- [ ] DAA - Decimal adjust after addition
- [ ] DAS - Decimal adjust after subtraction
- [ ] DEC - Decrement
- [ ] DIV - Divide
- [X] HLT - Halt
- [ ] IDIV
- [ ] IMUL - Integer multiply
- [ ] IN - Input
- [ ] INC - Increment
- [ ] INT - Interrupt
- [ ] INTO - Interrupt on overflow
- [ ] IRET - Interrupt return
- [ ] Jcc - Jump if condition
    - [ ] JA    
    - [ ] JAE
    - [ ] JB
    - [ ] JBE
    - [ ] JC
    - [ ] JCXZ
    - [ ] JE
    - [ ] JG
    - [ ] JGE
    - [ ] JL
    - [ ] JLE
    - [ ] JMP
    - [ ] JNA
    - [ ] JNAE
    - [ ] JNB
    - [ ] JNBE
    - [ ] JNC
    - [ ] JNE
    - [ ] JNG
    - [ ] JNGE
    - [ ] JNL       
    - [ ] JNLE
    - [ ] JNO
    - [ ] JNP
    - [ ] JNS
    - [ ] JNZ
    - [ ] JO
    - [ ] JP
    - [ ] JPE
    - [ ] JPO
    - [ ] JS
    - [ ] JZ
- [ ] LAHF - Load AH from flags
- [ ] LDS - Load DS from memory
- [ ] LEA - Load effective address
- [ ] LES - Load ES from memory
- [ ] LODS - Load string    
    - [ ] LODSB - Load string byte
    - [ ] LODSW - Load string word
- [ ] LOOP - Loop
- [ ] LOOPE - Loop if equal
- [ ] LOOPNE - Loop if not equal
- [ ] LOOPNZ - Loop if not zero
- [ ] LOOPZ - Loop if zero
- [ ] MOV - Move
    - [ ] MOVSB - Move string byte
    - [ ] MOVSW - Move string word
- [ ] MUL - Multiply
- [ ] NEG - Negate
- [X] NOP - No operation
- [ ] NOT - Not
- [ ] OR - Or
- [ ] OUT - Output
- [ ] POP - Pop
- [ ] POPA - Pop all
- [ ] POPF - Pop flags
- [ ] PUSH - Push
- [ ] PUSHA - Push all
- [ ] PUSHF - Push flags
- [ ] RCL - Rotate left through carry
- [ ] RCR - Rotate right through carry
- [ ] REP - Repeat
- [ ] REPE - Repeat if equal
- [ ] REPNE - Repeat if not equal
- [ ] REPNZ - Repeat if not zero
- [ ] REPZ - Repeat if zero
- [ ] RET - Return
- [ ] RETF - Return from far
- [ ] ROL - Rotate left
- [ ] ROR - Rotate right
- [ ] SAHF - Store AH from flags
- [ ] SAL - Shift arithmetic left
- [ ] SAR - Shift arithmetic right
- [ ] SBB - Subtract with borrow    
- [ ] SCAS - Scan string
    - [ ] SCASB - Scan string byte
    - [ ] SCASW - Scan string word
- [ ] SHL - Shift logical left
- [ ] SHR - Shift logical right
- [X] (STC)[#stc---set-carry-flag] - Set carry flag
- [X] STD - Set direction flag
- [X] STI - Set interrupt flag
- [ ] STOS - Store string
    - [ ] STOSB - Store string byte
    - [ ] STOSW - Store string word
- [ ] SUB - Subtract
- [ ] TEST - Test
- [ ] XCHG - Exchange
- [ ] XLATB - Translate byte
- [ ] XOR - Exclusive or

## Instruction by kind

### Data transfer instructions

#### General purpose

- MOV
- PUSH
- POP
- PUSHA
- POPA
- XCHG
- XLATB

#### Input/Output

- IN
- OUT

#### Address object and stack frame

- LEA
- LDS
- LES
- ENTER (added in 80186)
- LEAVE (added in 80186)

### Flag transfer

- LAHF
- SAHF
- PUSHF
- POPF

### Arithmetic instructions

#### Addition

- ADD
- ADC
- INC
- AAA
- DAA

#### Subtraction

- SUB
- SBB
- DEC
- NEG
- CMP
- AAS
- DAS

#### Multiplication

- MUL
- IMUL
- AAM

#### Division

- DIV
- IDIV
- AAD

#### Conversion

- CBW
- CWD

todo: maybe we can put aaa, aad, aam, aas in the same section ???

### Bit manipulation

#### Logical

- AND
- OR
- XOR
- NOT
- TEST

#### Shift

- SHL/SAL
- SHR
- SAR

#### Rotate

- ROL
- ROR
- RCL
- RCR

### String

- REP
- REPE/REPZ
- REPNE/REPNZ
- MOVSB/MOVSW
- CMPSB/CMPSW
- SCASB/SCASW
- LODSB/LODSW
- STOSB/STOSW

### Control instructions

#### Conditional

- Jcc

#### Unconditionnal

- CALL
- RET
- JMP

#### Iteration control

- LOOP
- LOOPE/LOOPZ
- LOOPNE/LOOPNZ
- JCXZ

#### Interrupt

- INT
- INTO
- IRET
- BOUND (introduced in 80286 ???)

### Processor control

#### Flag operations

- CLC
- CLD
- CLI
- CMC
- STC
- STD
- STI

#### External synchronization

- HLT
- WAIT
- ESC
- LOCK

#### No operation

- NOP

## Instructions details

### AAA - ASCII adjust after addition

Opcode: `37`

```
if (AL AND 0FH) > 9 or AF = 1 then:
    AL = AL + 6
    AH = AH + 1
    AF = 1
    CF = 1
else:
    AF = 0
    CF = 0
AL = AL AND 0FH
```

See example in [tests/isa/aaa.asm](../tests/isa/aaa.asm)

### AAD - ASCII adjust after division

Opcode: `D5 ib`

```
AL = (AL + AH * ib) AND 0FH
AH = 0
```

See example in [tests/isa/aad.asm](../tests/isa/aad.asm)

### AAM - ASCII adjust after multiplication

Opcode: `D4 ib`

```
tempAL = AL
AL = (tempAL * ib) AND 0FH
AH = tempAL / ib
```

See example in [tests/isa/aam.asm](../tests/isa/aam.asm)

### AAS - ASCII adjust after subtraction

Opcode: `3F`

```
if (AL AND 0FH) > 9 or AF = 1 then:
    AX = AX – 6
    AH = AH – 1
    AF = 1
    CF = 1
    AL = AL AND 0FH
else:
    AF = 0
    CF = 0
    AL = AL AND 0FH
```

See example in [tests/isa/aas.asm](../tests/isa/aas.asm)

### ADC - Add with carry

Opcodes: 

| bytes | mnemonic | note |
|-------|----------|------|
| `10 r/m` | ADC r/m8, r8 |  |
| `11 r/m` | ADC r/m16, r16 | |
| `12 r/m ib` | ADC r/m8, imm8 | |
| `13 r/m iw` | ADC r/m16, imm16 | |
| `14 ib` | ADC AL, imm8 | |
| `15 iw` | ADC AX, imm16 | |
| `80 /2 ib` | ADC r/m8, imm8 | |
| `81 /2 iw` | ADC r/m16, imm16 | |
| `82 /2 ib` | ADC r/m8, imm8 | todo: not in all x86 processors, same as `80 /2 ib` |
| `83 /2 iw` | ADC r/m16, imm16 | |

```
dst = dst + src + (CF ? 1 : 0)
```

See example in [tests/isa/adc.asm](../tests/isa/adc.asm)

### ADD - Add

Opcodes: 

| bytes | mnemonic |
|-------|----------|
| `00 r/m` | ADD r/m8, r8 |
| `01 r/m` | ADD r/m16, r16 |
| `02 r/m` | ADD r/m8, imm8 |
| `03 r/m` | ADD r/m16, imm16 |
| `04 ib` | ADD AL, imm8 |
| `05 iw` | ADD AX, imm16 |
| `04 ib` | ADC AL, imm8 | |
| `05 iw` | ADC AX, imm16 | |
| `80 /0 ib` | ADC r/m8, imm8 | |
| `81 /0 iw` | ADC r/m16, imm16 | |
| `82 /0 ib` | ADC r/m8, imm8 | todo: not in all x86 processors, same as `80 /0 ib` |
| `83 /0 iw` | ADC r/m16, imm16 | |

```
dst = dst + src
```

See example in [tests/isa/add.asm](../tests/isa/add.asm)

### AND - And

### CALL - Call

### CBW - Convert byte to word

### CLC - Clear carry flag

### CLD - Clear direction flag

### CLI - Clear interrupt flag

### CMC - Complement carry flag

### CMP - Compare

### CMPS - Compare strings

### CWD - Convert word to double word

### DAA - Decimal adjust after addition

### DAS - Decimal adjust after subtraction

### DEC - Decrement

### DIV - Divide

### HLT - Halt

### IDIV

### IMUL - Integer multiply

### IN - Input

### INC - Increment

### INT - Interrupt

### INTO - Interrupt on overflow

### IRET - Interrupt return

### Jcc - Jump if condition

### LAHF - Load AH from flags

### LDS - Load DS from memory

### LEA - Load effective address

### LES - Load ES from memory

### LODS - Load string    

### LOOP - Loop

### LOOPE - Loop if equal

### LOOPNE - Loop if not equal

### LOOPNZ - Loop if not zero

### LOOPZ - Loop if zero

### MOV - Move

### MUL - Multiply

### NEG - Negate

### NOP - No operation

### NOT - Not

### OR - Or

### OUT - Output

### POP - Pop

### POPA - Pop all

### POPF - Pop flags

### PUSH - Push

### PUSHA - Push all

### PUSHF - Push flags

### RCL - Rotate left through carry

### RCR - Rotate right through carry

### REP - Repeat

### REPE - Repeat if equal

### REPNE - Repeat if not equal

### REPNZ - Repeat if not zero

### REPZ - Repeat if zero

### RET - Return

### RETF - Return from far

### ROL - Rotate left

### ROR - Rotate right

### SAHF - Store AH from flags

### SAL - Shift arithmetic left

### SAR - Shift arithmetic right

### SBB - Subtract with borrow    

### SCAS - Scan string

### SHL - Shift logical left

### SHR - Shift logical right

### STC - Set carry flag

### STD - Set direction flag

### STI - Set interrupt flag

### STOS - Store string

### SUB - Subtract

### TEST - Test

### XCHG - Exchange

### XLATB - Translate byte

### XOR - Exclusive or

## Instruction by 

## Opcode map

### low nibble

|    | -0 | -1 | -2 | -3 | -4 | -5 | -6 | -7 |
|----|---|---|---|---|---|---|---|---|
| 0- | ADD r/m8, r8 | ADD r/m8, r8 | ADD r/m8, imm8 | ADD r/m8, imm16 | ADD AL, imm8 | ADD AX, imm16 | | |
| 1- | ADC r/m8, r8 | ADC r/m8, r8 | ADC r/m8, imm8 | ADC r/m8, imm16 | ADC AL, imm8 | ADC AX, imm16 | | |
| 2- | AND r/m8, r8 | AND r/m8, r8 | AND r/m8, imm8 | AND r/m8, imm16 | AND AL, imm8 | AND AX, imm16 | | |
| 3- | XOR r/m8, r8 | XOR r/m8, r8 | XOR r/m8, imm8 | XOR r/m8, imm16 | XOR AL, imm8 | XOR AX, imm16 | | AAA |
| 4- | | | | | | | | |
| 5- | | | | | | | | |
| 6- | | | | | | | | |
| 7- | | | | | | | | |

|    | -8 | -9 | -A | -B | -C | -D | -E | -F |
|----|---|---|---|---|---|---|---|---|
| 0- | OR r/m8, r8 | OR r/m8, r8 | OR r/m8, imm8 | OR r/m8, imm16 | OR AL, imm8 | OR AX, imm16 | | |
| 1- | SBB r/m8, r8 | SBB r/m8, r8 | SBB r/m8, imm8 | SBB r/m8, imm16 | SBB AL, imm8 | SBB AX, imm16 | | |
| 2- | SUB r/m8, r8 | SUB r/m8, r8 | SUB r/m8, imm8 | SUB r/m8, imm16 | SUB AL, imm8 | SUB AX, imm16 | | |
| 3- | CMP r/m8, r8 | CMP r/m8, r8 | CMP r/m8, imm8 | CMP r/m8, imm16 | CMP AL, imm8 | CMP AX, imm16 | | AAS |
| 4- | | | | | | | | |
| 5- | | | | | | | | |
| 6- | | | | | | | | |
| 7- | | | | | | | | |

### high nibble

|    | -0 | -1 | -2 | -3 | -4 | -5 | -6 | -7 |
|----|---|---|---|---|---|---|---|---|
| 8- | | | | | | | | |
| 9- | | | | | | | | |
| A- | | | | | | | | |
| B- | | | | | | | | |
| C- | | | | | | | | |
| D- | | | | | AAM imm8 | AAD imm8 | | |
| E- | | | | | | | | |
| F- | | | | | | | | |

|    | -8 | -9 | -A | -B | -C | -D | -E | -F |
|----|---|---|---|---|---|---|---|---|
| 8- | | | | | | | | |
| 9- | | | | | | | | |
| A- | | | | | | | | |
| B- | | | | | | | | |
| C- | | | | | | | | |
| D- | | | | | | | | |
| E- | | | | | | | | |
| F- | | | | | | | | |
