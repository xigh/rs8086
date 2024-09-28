%macro EXPECT   4-5
    ; start with line number
    DW      %2

    ; then the first call to this macro will output "^^" and the file name (0-terminated)
    %ifndef INSERT_LINENO_CALLED
        %define INSERT_LINENO_CALLED 1
        DB      "^^"
        DB      %1
        DB      0
        DW      %2
    %endif

    %ifdef DEBUG
    ; low byte
    %ifidn %3, AL
        DB      "AL", %4
    %elifidn %3, BL
        DB      "BL", %4
    %elifidn %3, CL
        DB      "CL", %4
    %elifidn %3, DL
        DB      "DL", %4
    ; high byte
    %elifidn %3, AH
        DB      "AH", %4
    %elifidn %3, BH
        DB      "BH", %4
    %elifidn %3, CH
        DB      "CH", %4
    %elifidn %3, DH
        DB      "DH", %4
    ; word
    %elifidn %3, AX
        DB      "AX"
        DW      %4
    %elifidn %3, BX
        DB      "BX"
        DW      %4
    %elifidn %3, CX
        DB      "CX"
        DW      %4
    %elifidn %3, DX
        DB      "DX"
        DW      %4
    %elifidn %3, SI
        DB      "SI"
        DW      %4
    %elifidn %3, DI
        DB      "DI"
        DW      %4
    %elifidn %3, BP
        DB      "BP"
        DW      %4
    %elifidn %3, SP
        DB      "SP"
        DW      %4
    ; segment
    %elifidn %3, CS
        DB      "CS"
        DW      %4
    %elifidn %3, DS
        DB      "DS"
        DW      %4
    %elifidn %3, ES
        DB      "ES"
        DW      %4
    %elifidn %3, SS
        DB      "SS"
        DW      %4
    ; flags
    %elifidn %3, CF
        DB      "CF", %4
    %elifidn %3, PF
        DB      "PF", %4
    %elifidn %3, AF
        DB      "AF", %4
    %elifidn %3, ZF
        DB      "ZF", %4
    %elifidn %3, SF
        DB      "SF", %4
    %elifidn %3, OF
        DB      "OF", %4
    %elifidn %3, IF
        DB      "IF", %4
    %elifidn %3, DF
        DB      "DF", %4
    %elifidn %3, TF
        DB      "TF", %4
    ; memory
    %elifidn %3, MB
        DB      "MB"
        DD      %4
        DB      %5
    %elifidn %3, MW
        DB      "MW"
        DD      %4
        DW      %5
    %elifidn %3, MD
        DB      "MD"
        DD      %4
        DD      %5
    %elifidn %3, MQ
        DB      "MQ"
        DD      %4
        DQ      %5
    %else
        %error "unknown operand"
    %endif
    %endif
%endmacro
