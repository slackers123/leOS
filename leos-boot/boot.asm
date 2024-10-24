; Simple bootloader example for x86 systems that should print out a simple message to the user

bits 16     ; We're dealing with 16 bit code
org 0x7c00  ; Inform the assembler of the starting location for this code

boot:
    mov si, message ; Point SI register to message
    mov ah, 0x0e    ; Set higher bits to the display character command

.loop:
    lodsb       ; Load the character within the AL register, and increment SI
    cmp al, 0   ; Is the AL register a null byte?
    je halt     ; Jump to halt
    int 0x10    ; Trigger video services interrupt
    jmp .loop   ; Loop again

halt:
    hlt         ; Stop

message:
    db "Howdy!", 0

; Mark the device as bootable
times 510-($-$$) db 0 ; Add any additional zeroes to make 510 bytes in total
dw 0xAA55 ; Write the final 2 bytes as the magic number 0x55aa, remembering x86 little endian
