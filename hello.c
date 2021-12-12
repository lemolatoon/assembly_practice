void print(const void* c);

int main(void) {
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
	return 0;
}
