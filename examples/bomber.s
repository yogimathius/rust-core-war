.name "Bomber"
.comment "Aggressive carpet bomber"

start:
    live %1
    add %10, ptr, ptr
    mov %0, ptr
    live %1
    add %15, ptr, ptr
    mov %0, ptr
    live %1
    sub %5, ptr, ptr
    mov %0, ptr
    jmp start

ptr: live %0