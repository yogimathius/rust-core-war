;name Fortress
;comment Defensive champion that builds barriers and repairs itself

start:
    live %1              ; Declare we're alive
    mov #start, r1       ; Self-reference for repair
    mov #50, r2          ; Barrier distance
    
build_barriers:
    live %1              ; Stay alive
    mov #gate, @r2       ; Build defensive gate
    add #25, r2          ; Next barrier position
    mov #gate, @r2       ; Another gate
    sub #50, r2          ; Other direction
    mov #gate, @r2       ; Gate on other side
    add #25, r2          ; Return to center
    
    ; Self-repair check
    cmp.i @start, #live  ; Check if our start is intact
    je continue_defense  ; If intact, continue
    mov #live, @start    ; Repair ourselves
    
continue_defense:
    fork patrol          ; Create patrol process
    mov #100, r3         ; Counter for next phase
    
wait_loop:
    live %1              ; Stay alive during wait
    sub #1, r3           ; Decrement counter
    jnz wait_loop        ; Wait a bit
    jmp build_barriers   ; Rebuild barriers
    
patrol:
    live %1              ; Patrol process
    mov #200, r4         ; Patrol distance
    
patrol_loop:
    live %1              ; Stay alive
    cmp.i @r4, #0        ; Check for enemies
    je patrol_continue   ; If empty, continue
    mov #0, @r4          ; Bomb enemy
    mov #0, @r4+1        ; Bomb next location too
    
patrol_continue:
    add #10, r4          ; Move patrol position
    cmp r4, #400         ; Reached patrol limit?
    jl patrol_loop       ; Continue patrol
    mov #200, r4         ; Reset patrol
    jmp patrol_loop      ; Keep patrolling
    
gate:
    ; Defensive code - redirects enemies
    live %1
    jmp -10              ; Send enemies backwards