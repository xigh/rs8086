CPU     8086
BITS    16
%include "./expect.inc"

_start:
        HLT ; should wait for an interrupt, but in test mode, we just stop here

        EXPECT  __FILE__, __LINE__, AX, 0x0000
        EXPECT  __FILE__, __LINE__, BX, 0x0000
        EXPECT  __FILE__, __LINE__, CX, 0x0000
        EXPECT  __FILE__, __LINE__, DX, 0x0000
        EXPECT  __FILE__, __LINE__, SI, 0x0000
        EXPECT  __FILE__, __LINE__, DI, 0x0000
        EXPECT  __FILE__, __LINE__, BP, 0x0000
        EXPECT  __FILE__, __LINE__, SP, 0x0000

        EXPECT  __FILE__, __LINE__, CS, 0xF000
        EXPECT  __FILE__, __LINE__, DS, 0x0000
        EXPECT  __FILE__, __LINE__, ES, 0x0000
        EXPECT  __FILE__, __LINE__, SS, 0x0000

        EXPECT  __FILE__, __LINE__, CF, 0
        EXPECT  __FILE__, __LINE__, PF, 0
        EXPECT  __FILE__, __LINE__, AF, 0
        EXPECT  __FILE__, __LINE__, ZF, 0
        EXPECT  __FILE__, __LINE__, SF, 0
        EXPECT  __FILE__, __LINE__, TF, 0
        EXPECT  __FILE__, __LINE__, IF, 0
        EXPECT  __FILE__, __LINE__, DF, 0
        EXPECT  __FILE__, __LINE__, OF, 0
