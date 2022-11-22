#include "../common/bignum/bignum.hpp"
#include <iostream>

BigNum bignum_pow(const size_t base, const size_t exponent) {
    BigNum sum = split_to_bignum(base);
    for (size_t i = 1; i < exponent; i++) {
        sum = bignum_mult(sum, base);
    }
    return sum;
}

int main(void) {
    std::cout << sum_bignum_digits(bignum_pow(2, 1000)) << '\n';
}
