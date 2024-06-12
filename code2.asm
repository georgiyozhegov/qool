section .bss
	y: resq 1
	x: resq 1
	z: resq 1
section .text
	global _start
_start:
	mov rax, 10
	mov QWORD [y], rax
	mov rax, 2
	mov rbx, rax
	mov rax, 3
	add rax, rbx
	mov rbx, rax
	mov rax, 4
	sub rax, rbx
	mov QWORD [x], rax
	mov rax, [x]
	mov rbx, rax
	mov rax, 2
	mov rbx, rax
	mov rax, [x]
	add rax, rbx
	sub rax, rbx
	mov QWORD [z], rax
	mov rax, 60
	mov rdi, 0
	syscall

