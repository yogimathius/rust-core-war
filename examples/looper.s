.name "Looper"
.comment "Simple champion that loops with live instructions"

start:
    live %1
    jmp start