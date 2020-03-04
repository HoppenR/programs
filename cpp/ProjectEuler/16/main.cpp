#include <cmath>
#include <cstdint>
#include <iostream>
#include <string>
#include <vector>

// TODO: optimize this

uint8_t getn(const size_t number, const uint8_t n) {
	// returns the nth digit in number, counted from right hand side, 0-based
	return number / static_cast<uint8_t>(std::pow(10, n)) % 10;
}

std::vector<uint8_t> bignum_pow(const size_t base, const size_t exponent) {
	std::vector<uint8_t> sum;
	const float lgBase = log10f(base);
	if (!std::isinf(lgBase)) {
		for (uint8_t i = 0; i <= static_cast<uint8_t>(lgBase); i++) {
			sum.insert(sum.begin(), getn(base, i));
		}
	}
	for (size_t i = 1; i < exponent; i++) {
		size_t carry = 0;
		for (size_t j = 0; j < sum.size(); j++) {
			size_t product = sum.at(sum.size() - 1 - j) * base + carry;
			sum.at(sum.size() - 1 - j) = product % 10;
			carry = product / 10;
		}
		const float lgCarry = log10f(carry);
		if (!std::isinf(lgCarry)) {
			for (uint8_t j = 0; j <= static_cast<uint8_t>(lgCarry); j++) {
				sum.insert(sum.begin(), getn(carry, j));
			}
		}
	}
	return sum;
}

size_t sum_bignum_digits(const std::vector<uint8_t>& number) {
	size_t sum = 0;
	for (const uint8_t n : number) {
		sum += n;
	}
	return sum;
}

int main(void) {
	std::cout << sum_bignum_digits(bignum_pow(2, 1000)) << '\n';
}
