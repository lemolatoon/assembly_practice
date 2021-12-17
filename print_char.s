.intel_syntax noprefix
.global print_char

.data
char:
	.byte 0
	.align 2

str:
	.byte 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0
	.align 2

.text
print_char:
	mov [char + rip], rdi	# mov char from resistor to memory
	mov rax, 0x1			# write
	mov rdi, 0x1			# file discriptor
	lea rsi, [char + rip]	# address of char
	mov rdx, 0x1			# length
	syscall
	ret

/*
prints:
	# rdi : const void * (char)
	# rsi : str_length
	mov rcx, 0				# file discriptor
prints_loop:
	add rax, str
	add rax, rcx
	mov [rax], rdi
	cmp rcx, rsi
	je prints_call
	inc rcx
	jmp prints_loop
prints_call:
	mov rdx, rsi			# length
	lea rsi, [str + rip]	# pointer of first address
	mov rax, 0x1			# write
	syscall
	ret
*/
	
