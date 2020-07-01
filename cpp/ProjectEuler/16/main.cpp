#include "../common/bignum/bignum.hpp"
#include <iostream>

std::vector<uint8_t> bignum_pow(const size_t base, const size_t exponent) {
	std::vector<uint8_t> sum;
	const float lgBase = log10f(base);
	if (!std::isinf(lgBase)) {
		for (uint8_t i = 0; i <= static_cast<uint8_t>(lgBase); i++) {
			sum.insert(sum.begin(), getn(base, i));
		}
	}
	for (size_t i = 1; i < exponent; i++) {
		sum = bignum_mult(sum, base);
	}
	return sum;
}

int main(void) {
	std::cout << sum_bignum_digits(bignum_pow(2, 1000)) << '\n';
}
