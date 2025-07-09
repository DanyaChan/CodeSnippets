#include <iostream>
#include <utility>
bool cond(int num) {
    int sum = 0;
    for (int i = 0; i < num; i++) {
        sum += i * i;
    }
    return sum % 13 == 0 || true;
}

int some(int n) {
    if (cond(n)) {
        int x = x;
        return x * x + 100;
    }
    else {
        return 1000;
    }
}

int main() {
    std::cout << some(14);
}