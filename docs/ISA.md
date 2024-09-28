# 8086 ISA

## operands

- GENERAL PURPOSE REGISTERS:  AH, AL, BL, BH, CH, CL, DH, DL,
        AX, BX, CX, DX, DI, SI, BP, SP.

- FLAGS: CF, PF, AF, ZF, SF, TF, IF, DF, OF.

- SEGMENT REGISTERS: DS, ES, SS, and only as second operand: CS.

- MEMORY ADDRESSING: [BX+SI], [BX+DI], [BP+SI], [BP+DI], [SI], [DI], [BP], [BX] (todo)

- IMMEDIATE VALUE: 8 or 16 bits : 0x12, 0x1234, etc.

## 16-bit instructions

- [ ] [AAA](#aaa---ascii-adjust-after-addition) - ASCII adjust after addition
- [ ] [AAD](#aad---ascii-adjust-after-division) - ASCII adjust after division
- [ ] AAM - ASCII adjust after multiplication
- [ ] AAS - ASCII adjust after subtraction
- [ ] ADC - Add with carry
- [ ] ADD - Add
- [ ] AND - And
- [ ] CALL - Call
- [ ] CBW - Convert byte to word
- [ ] CLC - Clear carry flag
- [ ] CLD - Clear direction flag
- [ ] CLI - Clear interrupt flag
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
- [ ] HLT - Halt
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
- [ ] NOP - No operation
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
- [ ] STC - Set carry flag
- [ ] STD - Set direction flag
- [ ] STI - Set interrupt flag
- [ ] STOS - Store string
    - [ ] STOSB - Store string byte
    - [ ] STOSW - Store string word
- [ ] SUB - Subtract
- [ ] TEST - Test
- [ ] XCHG - Exchange
- [ ] XLATB - Translate byte
- [ ] XOR - Exclusive or

## 8086 instructions

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

Opcode: `D5`

```
AL = (AL + AH * 10) AND 0FH
AH = 0
```

See example in [tests/isa/aad.asm](../tests/isa/aad.asm)

### AAM - ASCII adjust after multiplication

Opcode: `D4`

```
AL = (AL * AH) AND 0FH
AH = 0
```

See example in [tests/isa/aam.asm](../tests/isa/aam.asm)

### AAS - ASCII adjust after subtraction

Opcode: `3F`

```
if (AL AND 0FH) > 9 or AF = 1 then:
    AL = AL - 6
    AH = AH - 1
    AF = 1
    CF = 1
else:
    AF = 0
    CF = 0
AL = AL AND 0FH
```

See example in [tests/isa/aas.asm](../tests/isa/aas.asm)

### ADC - Add with carry

Opcodes: 

| bytes | mnemonic | description |
|-------|----------|-------------|
| `10 op r/m` | ADC r/m8, r8 | Add with carry |
| `11 op r/m` | ADC r/m16, r16 | Add with carry |
| `12 op r/m` | ADC r/m8, imm8 | Add with carry |
| `13 op r/m` | ADC r/m16, imm16 | Add with carry |
| `14` | ADC AL, imm8 | Add with carry |
| `15` | ADC AX, imm16 | Add with carry |

todo

```
AL = AL + (CF ? 1 : 0)
```

See example in [tests/isa/adc.asm](../tests/isa/adc.asm)

### ADD - Add

Opcodes: 

| bytes | mnemonic | description |
|-------|----------|-------------|
| `00 op r/m` | ADD r/m8, r8 | Add |
| `01 op r/m` | ADD r/m16, r16 | Add |
| `02 op r/m` | ADD r/m8, imm8 | Add |
| `03 op r/m` | ADD r/m16, imm16 | Add |
| `04` | ADD AL, imm8 | Add |
| `05` | ADD AX, imm16 | Add |

todo

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

## Opcode map (todo)

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
| D- | | | | | AAM | AAD | | |
| E- | | | | | | | | |
| F- | | | | | | | | |

|    | 8 | 9 | A | B | C | D | E | F |
|----|---|---|---|---|---|---|---|---|
| 8- | | | | | | | | |
| 9- | | | | | | | | |
| A- | | | | | | | | |
| B- | | | | | | | | |
| C- | | | | | | | | |
| D- | | | | | | | | |
| E- | | | | | | | | |
| F- | | | | | | | | |
