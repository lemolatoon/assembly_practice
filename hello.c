#define len(s) (sizeof(s) / sizeof(char))

void print(const void* c);
void print_char(char c);
// void prints(const void * c, int str_length);


void println(char* str, int num_len) {
	for (int i = 0; i < num_len; i++) {
		print_char(str[i]);
	}
	print_char("\n"[0]);
}

int hello(void) {
	char str[] = "Hello World";
	print(&"a");
	print(&"a");
	print(&"a");
	print(&"\n");
	print(&str);
	print(&"\n");
	for (int i = 0; i < sizeof(str) / sizeof(char); i++) {
		print(&str[i]);
	}
	print(&"\n");
	print_char("h"[0]);
	print_char("\n"[0]);
	char str2[] = "Hello from `print_char`!!";
	print_char("h"[0]);
	println(str2, len(str2));
	char str3[] = "Hello from `prints`!!\n";
	// prints(&str3, len(str3));
	return 0;
}


