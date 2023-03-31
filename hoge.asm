BITS 32
start:
    org 0x7c00
    mov eax, 10
    mov dword [eax], 20
    mov ebx, [eax]
    jmp 0