#include "../common/bignum/bignum.hpp"
#include <iostream>

BigNum bignum_factorial(const size_t start) {
	BigNum sum = split_to_bignum(start);
	for (int i = start - 1; i > 0; i--) {
		sum = bignum_mult(sum, i);
	}
	return sum;
}

int main(void) {
	std::cout << sum_bignum_digits(bignum_factorial(100)) << '\n';
}
