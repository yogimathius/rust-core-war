;name Quicksilver
;comment Fast-moving champion that uses hit-and-run tactics

start:
    live %1              ; Declare we're alive
    mov #attack_pattern, r1 ; Load attack pattern
    mov #50, r2          ; Initial attack distance
    
rapid_strike:
    live %1              ; Stay alive
    fork fast_bomber     ; Create fast attack process
    add #100, r2         ; Move attack position
    
    ; Quick scan and strike
    cmp.i @r2, #0        ; Check target
    je skip_target       ; Skip if empty
    mov #0, @r2          ; Quick bomb
    mov #0, @r2+1        ; Follow-up bomb
    mov #0, @r2-1        ; Surrounding bomb
    
skip_target:
    add #37, r2          ; Prime number jump (harder to predict)
    cmp r2, #800         ; Reached limit?
    jl rapid_strike      ; Continue if not
    
    ; Reset for another pass
    mov #50, r2          ; Reset position
    fork evasive_maneuver ; Create evasion process
    jmp rapid_strike     ; Attack again
    
fast_bomber:
    live %1              ; Fast bombing process
    mov #200, r3         ; Bombing start
    mov #3, r4           ; Bomb count
    
bomb_burst:
    live %1              ; Stay alive
    mov #0, @r3          ; Drop bomb
    add #11, r3          ; Move position (prime number)
    sub #1, r4           ; Decrement counter
    jnz bomb_burst       ; Continue burst
    
    ; Quick escape
    add #200, r3         ; Jump far away
    mov #5, r4           ; Another burst
    jmp bomb_burst       ; Keep bombing
    
evasive_maneuver:
    live %1              ; Evasion process
    mov #start, r5       ; Self-location
    mov #1000, r6        ; Escape distance
    
escape_loop:
    live %1              ; Stay alive during escape
    mov @r5, @r6         ; Copy self to new location
    add #1, r5           ; Next instruction
    add #1, r6           ; Next destination
    cmp r5, #evasive_maneuver ; Copied enough?
    jl escape_loop       ; Continue copying
    
    fork @1000           ; Activate copy at new location
    jmp start            ; Original continues

attack_pattern:
    ; Attack pattern data
    live %1
    mov #0, @100         ; Systematic bombing pattern
    mov #0, @200
    mov #0, @300