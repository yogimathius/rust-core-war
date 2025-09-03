;name Replicator
;comment Copies itself to multiple locations and spreads

start:
    live %1              ; Declare we're alive
    mov #copy_start, r1  ; Source address
    mov #500, r2         ; First destination
    mov #copy_end, r3    ; End of code marker
    sub r1, r3           ; Calculate code size
    
copy_loop:
    live %1              ; Stay alive during copy
    mov @r1, @r2         ; Copy one instruction
    add #1, r1           ; Next source
    add #1, r2           ; Next destination
    sub #1, r3           ; Decrement counter
    jnz copy_loop        ; Continue if more to copy
    
    fork @500            ; Start copy at new location
    
    ; Prepare for another copy
    mov #copy_start, r1  ; Reset source
    add #200, r2         ; Next destination (500 + 200 = 700)
    mov #copy_end, r3    ; Reset size counter
    sub r1, r3
    jmp copy_loop        ; Make another copy
    
copy_start:
    ; This marks the start of copyable code
    live %1
    mov #10, r4          ; Bomb counter
bomb_loop:
    live %1
    mov #0, @r4          ; Drop bomb
    add #10, r4          ; Move bombing position
    sub #1, r4           ; Decrement counter (this creates a pattern)
    jnz bomb_loop        ; Continue bombing
    jmp copy_start       ; Restart
copy_end:
    ; End marker (not executed)