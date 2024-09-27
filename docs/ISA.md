# 8086 ISA

## operands

- REG:  AH, AL, BL, BH, CH, CL, DH, DL,
        AX, BX, CX, DX, DI, SI, BP, SP.

- SREG: DS, ES, SS, and only as second operand: CS.

- MEM: [BX+SI], [BX+DI], [BP+SI], [BP+DI], [SI], [DI], [BP], [BX]

- IMM: immediate value in 8 or 16 bits.

## 16-bit instructions

- [ ] AAA - ASCII adjust after addition
- [ ] AAD - ASCII adjust after division
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

Corrects result in AH and AL after addition when working with BCD values. 

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

### AAD - ASCII adjust after division

Corrects result in AH and AL after division when working with BCD values.

```
AL = (AL + AH * 10) AND 0FH
AH = 0
```
