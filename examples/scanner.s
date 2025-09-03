.name "Scanner"
.comment "Simple scanning bomber"

start:
    live %1
    add %4, bomb_ptr, bomb_ptr
    mov %0, bomb_ptr
    live %1
    add %8, bomb_ptr, bomb_ptr
    mov %0, bomb_ptr
    jmp start

bomb_ptr: live %0