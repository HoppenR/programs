#include "../common/bignum/bignum.hpp"
#include <iostream>

std::vector<uint8_t> bignum_factorial(const size_t start) {
	std::vector<uint8_t> sum;
	const float lgBase = log10f(start);
	if (!std::isinf(lgBase)) {
		for (uint8_t i = 0; i <= static_cast<uint8_t>(lgBase); i++) {
			sum.insert(sum.begin(), getn(start, i));
		}
	}
	for (int i = start - 1; i > 0; i--) {
		sum = bignum_mult(sum, i);
	}
	return sum;
}

int main(void) {
	std::cout << sum_bignum_digits(bignum_factorial(100)) << '\n';
}
