CPU     8086
BITS    16
ORG     0       ; !!! hey: in fact, we start at 0xf000:0x0000
%include "expect.inc"

_start:
        MOV     SP, 0xfffe
        MOV     AX, 0
        MOV     SS, AX

        CALL    .test_r8_r8
        CALL    .test_m8_r8
        HLT

.test_r8_r8:
        MOV     CL, 0x01
        MOV     BL, 0x02
        ADD     BL, CL
        RET
        HLT

        EXPECT  __FILE__, __LINE__, BL, 3
        EXPECT  __FILE__, __LINE__, DONE

.test_m8_r8_val:
        DB      4

.test_m8_r8:
        MOV     BL, 1
        ADD     [.test_m8_r8_val], BL
        RET
        HLT

        EXPECT  __FILE__, __LINE__, MB, .test_m8_r8_val, 5
        EXPECT  __FILE__, __LINE__, DONE

.todo:
        MOV AL, 0x05
        AND AL, 0x01
        HLT

; EXPECT AX == 0x01
