.global _start

_start:
    call main
	mov $60, %eax 		# exit
	xor %rdi, %rdi 		# mov $0, %rdi
	syscall
