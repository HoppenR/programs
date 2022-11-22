#include <cmath>
#include <iostream>

bool is_prime(const int num) {
    const int target = std::sqrt(num);
    for (int i = 2; i <= target; i++) {
        if (num % i == 0) {
            return false;
        }
    }
    return true;
}

int find_nth_prime(const int num) {
    if (num == 1)
        return 2;
    int primesfound = 1; // 2
    int ans = 3;
    while (primesfound < num) {
        if (is_prime(ans)) {
            primesfound++;
        }
        ans += 2;
    }
    return ans - 2;
}

int main(void) {
    std::cout << find_nth_prime(10001) << '\n';
}
