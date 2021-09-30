// This file is a part of Simple-XX/SimpleCompiler
// (https://github.com/Simple-XX/SimpleCompiler).
//
// test_tyck.c for Simple-XX/SimpleCompiler.

int global = 1;

int fib(int a) {
    if (a == 0)
        return 1;
    if (a == 1)
        return 2;
    return fib(a - 1) + fib(a - 2);
}

int main() {
    int a = global + 1;
    a = a + fib(a);
    return 0;
}