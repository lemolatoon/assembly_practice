## 実行方法

```
C言語を通して実行
$ gcc -nostdlib substart.s print_b.s print_char.s hello.c -o hello
$ ./hello
```
```
Rustを通して実行
$ rustc hello.rs -C panic=abort --emit obj
$ gcc -nostdlib hello.o substart.s print_b.s print_char.s -o rhello
$ ./rhello
```
