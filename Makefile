compile:
	rm code.asm
	cargo run >> code.asm
	nasm -g -f elf64 code.asm -o code.o && ld -g code.o -o code

run: compile
	./code

debug: compile
	gdb code

