.global print


print:
	mov $0, %rcx
	jmp loop

loop:
	mov %rdi, %rsi
	mov $1, %rdi		# file discripter
	mov $1, %rdx		# size of char
	mov $1, %eax		# write
	#pushl %rcx			# `q` for 64bit
	syscall
	#popl %rcx
	inc %rcx
	cmp $0x10, %rcx
	je loop
	ret

exit:
	mov $0x3c, %eax		# write
	syscall
	

