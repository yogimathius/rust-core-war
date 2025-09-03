;name Vampire
;comment Converts enemy code into copies of itself

start:
    live %1              ; Declare we're alive
    mov #search_start, r1 ; Where to start searching
    mov #vampire_code, r2 ; Our conversion template
    
search_phase:
    live %1              ; Stay alive
    cmp.i @r1, #0        ; Is this location empty?
    je next_search       ; Skip empty locations
    cmp.i @r1, #live     ; Is this enemy code?
    je convert_enemy     ; Convert enemy live instructions
    cmp.i @r1, #mov      ; Is this a mov instruction?
    je convert_enemy     ; Convert it too
    
next_search:
    add #1, r1           ; Next location
    cmp r1, #1000        ; Searched enough?
    jl search_phase      ; Continue searching
    mov #search_start, r1 ; Reset search
    jmp search_phase     ; Search again
    
convert_enemy:
    live %1              ; Stay alive during conversion
    mov vampire_code, @r1     ; Convert first instruction
    add #1, r1           ; Next location  
    mov vampire_code+1, @r1   ; Convert second instruction
    add #1, r1           ; Next location
    mov vampire_code+2, @r1   ; Convert third instruction
    
    fork @r1-2           ; Activate converted code
    jmp search_phase     ; Continue hunting
    
vampire_code:
    live %1              ; Converted code template
    fork vampire_spawn   ; Create more vampires
    jmp search_for_more  ; Hunt for more victims
    
vampire_spawn:
    live %1              ; New vampire process
    mov #300, r3         ; Bombing distance
bomb_enemies:
    live %1              ; Stay alive
    mov #0, @r3          ; Drop bomb
    add #7, r3           ; Move bombing position
    jmp bomb_enemies     ; Keep bombing
    
search_for_more:
    live %1              ; Original vampire continues
    add #50, r1          ; Jump ahead in search
    jmp search_phase     ; Continue conversion mission

search_start:
    ; Search starting point
    live %1