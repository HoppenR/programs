#include "./bignum.hpp"
uint8_t getn(const size_t number, const uint8_t n) {
	// returns the nth digit in number, counted from right hand side, 0-based
	return number / static_cast<uint8_t>(std::pow(10, n)) % 10;
}

std::vector<uint8_t> bignum_mult(std::vector<uint8_t> sum,
								 const uint8_t multer) {
	size_t carry = 0;
	for (size_t j = 0; j < sum.size(); j++) {
		size_t product = sum.at(sum.size() - 1 - j) * multer + carry;
		sum.at(sum.size() - 1 - j) = product % 10;
		carry = product / 10;
	}
	const float lgCarry = log10f(carry);
	if (!std::isinf(lgCarry)) {
		for (uint8_t j = 0; j <= static_cast<uint8_t>(lgCarry); j++) {
			sum.insert(sum.begin(), getn(carry, j));
		}
	}
	return sum;
}
int sum_bignum_digits(const std::vector<uint8_t>& number) {
	return std::accumulate(number.begin(), number.end(), 0);
}
