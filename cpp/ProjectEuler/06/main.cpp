#include <cmath>
#include <iostream>

long sum_square_difference(long num) {
    num = std::abs(num);
    const long sum = (num * (num + 1) / 2);
    const long sumSquared = std::pow(sum, 2);
    long squaresSum = 0;
    for (long i = 1; i <= num; i++) {
        squaresSum += std::pow(i, 2);
    }
    return sumSquared - squaresSum;
}

int main(void) {
    std::cout << sum_square_difference(100) << '\n';
}
