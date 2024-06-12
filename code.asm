section .data
	x: dq 10
section .bss
	y: resq 1
section .text
	global _start
_start:
	mov rax, [x]
	mov rbx, 3
	add rax, rbx
	mov QWORD [y], rax

	mov rax, 60
	mov rdi, 0
	syscall

