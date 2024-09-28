%macro EXPECT   2-3
    %ifdef DEBUG
    ; low byte
    %ifidn %1, AL
        DB      "AL", %2
    %elifidn %1, BL
        DB      "BL", %2
    %elifidn %1, CL
        DB      "CL", %2
    %elifidn %1, DL
        DB      "DL", %2
    ; high byte
    %elifidn %1, AH
        DB      "AH", %2
    %elifidn %1, BH
        DB      "BH", %2
    %elifidn %1, CH
        DB      "CH", %2
    %elifidn %1, DH
        DB      "DH", %2
    ; word
    %elifidn %1, AX
        DB      "AX"
        DW      %2
    %elifidn %1, BX
        DB      "BX"
        DW      %2
    %elifidn %1, CX
        DB      "CX"
        DW      %2
    %elifidn %1, DX
        DB      "DX"
        DW      %2
    %elifidn %1, SI
        DB      "SI"
        DW      %2
    %elifidn %1, DI
        DB      "DI"
        DW      %2
    %elifidn %1, BP
        DB      "BP"
        DW      %2
    %elifidn %1, SP
        DB      "SP"
        DW      %2
    ; segment
    %elifidn %1, CS
        DB      "CS"
        DW      %2
    %elifidn %1, DS
        DB      "DS"
        DW      %2
    %elifidn %1, ES
        DB      "ES"
        DW      %2
    %elifidn %1, SS
        DB      "SS"
        DW      %2
    ; flags
    %elifidn %1, CF
        DB      "CF", %2
    %elifidn %1, PF
        DB      "PF", %2
    %elifidn %1, AF
        DB      "AF", %2
    %elifidn %1, ZF
        DB      "ZF", %2
    %elifidn %1, SF
        DB      "SF", %2
    %elifidn %1, OF
        DB      "OF", %2
    %elifidn %1, IF
        DB      "IF", %2
    %elifidn %1, DF
        DB      "DF", %2
    %elifidn %1, TF
        DB      "TF", %2
    ; memory
    %elifidn %1, MB
        DB      "MB"
        DD      %2
        DB      %3
    %elifidn %1, MW
        DB      "MW"
        DD      %2
        DW      %3
    %elifidn %1, MD
        DB      "MD"
        DD      %2
        DD      %3
    %elifidn %1, MQ
        DB      "MQ"
        DD      %2
        DQ      %3
    %else
        %error "unknown operand"
    %endif
    %endif
%endmacro
