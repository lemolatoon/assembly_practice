.intel_syntax noprefix
.global _start

.text
string:
	.ascii "a"

_start:
    call hello
	lea rdi, [string + rip]  # rip relative
	call print
	mov rax, 60 		# exit
	xor rdi, rdi 		# mov $0, %rdi
	syscall

