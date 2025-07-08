; Dwarf - simple bomber
.name "Dwarf"
.comment "Throws bombs every 4 cells"

start:
    add %4, bomb_ptr, bomb_ptr
    mov %0, bomb_ptr
    jmp start

bomb_ptr: live %0 