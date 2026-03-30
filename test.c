// heavy_no_libs.c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Больше функций, чтобы компилятор мог их инлайнить или вырезать
static int add(int a, int b) { return a + b; }
static int sub(int a, int b) { return a - b; }
static int mul(int a, int b) { return a * b; }
static int div_int(int a, int b) { return b ? a / b : 0; }

// Бесполезные функции, которые оптимизатор может вырезать
static int useless_calc(int x) {
    int res = 0;
    for (int i = 0; i < 100; i++) {
        res += x * i;
        res -= x / (i + 1);
    }
    return res;
}

// Рекурсивная функция, которую можно оптимизировать
int factorial(int n) {
    if (n <= 1) return 1;
    return n * factorial(n - 1);
}

// Функция с большим количеством операций
long long heavy_loop(int n) {
    long long sum = 0;
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            sum += add(i, j) * sub(i, j) + mul(i, j) / (div_int(j, i + 1) + 1);
        }
    }
    return sum;
}

// Массив функций для косвенных вызовов (мешает оптимизации)
typedef int (*operation)(int, int);
operation ops[] = {add, sub, mul, div_int};

int main(int argc, char *argv[]) {
    volatile int result = 0;  // volatile мешает оптимизации
    
    // Используем аргументы, чтобы компилятор не вырезал всё
    int iterations = (argc > 1) ? atoi(argv[1]) : 1000;
    
    // 1. Циклы с вызовами функций
    for (int i = 0; i < iterations; i++) {
        for (int j = 0; j < 100; j++) {
            result += add(i, j);
            result ^= sub(i, j);
            result |= mul(i, j);
            result &= div_int(i, j + 1);
        }
    }
    
    // 2. Косвенные вызовы через массив (трудно оптимизировать)
    for (int i = 0; i < 1000; i++) {
        int op_idx = i % 4;
        result += ops[op_idx](i, i + 1);
    }
    
    // 3. Рекурсия
    result += factorial(10);
    
    // 4. Бесполезные вычисления (могут быть вырезаны)
    result += useless_calc(42);
    
    // 5. Тяжёлый цикл
    result += heavy_loop(100);
    
    // Выводим результат, чтобы компилятор не вырезал всё
    printf("Result: %d\n", result);
    
    return 0;
}
