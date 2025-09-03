.name "Survivor"
.comment "Defensive champion with evasive tactics"

start:
    live %1
    fork safe_copy
    add %20, scan_ptr, scan_ptr
    mov %0, scan_ptr
    live %1
    add %30, scan_ptr, scan_ptr
    mov %0, scan_ptr
    jmp start

safe_copy:
    live %1
    add %100, backup_ptr, backup_ptr
    mov start, backup_ptr
    live %1
    fork backup_ptr
    jmp start

scan_ptr: live %0
backup_ptr: live %0