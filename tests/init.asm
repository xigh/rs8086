CPU     8086
BITS    16
%include "./expect.inc"

_start:
        HLT ; should wait for an interrupt, but in test mode, we just stop here

        EXPECT  AX, 0x0000
        EXPECT  BX, 0x0000
        EXPECT  CX, 0x0000
        EXPECT  DX, 0x0000
        EXPECT  SI, 0x0000
        EXPECT  DI, 0x0000
        EXPECT  BP, 0x0000
        EXPECT  SP, 0x0000

        EXPECT  CS, 0xF000
        EXPECT  DS, 0x0000
        EXPECT  ES, 0x0000
        EXPECT  SS, 0x0000

        EXPECT  CF, 0
