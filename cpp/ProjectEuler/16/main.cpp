#include <iostream>
#include <string>

constexpr int ChrDecDiff = 48; // 1 + 48 = '1'

std::string bignum_pow(const size_t base, const size_t exponent) {
	std::string sum = std::to_string(base);
	for (size_t i = 1; i < exponent; i++) {
		short carry = 0;
		for (size_t j = 0; j < sum.size(); j++) {
			short digit = (sum.at(sum.size() - 1 - j) - ChrDecDiff);
			digit *= base;
			digit += carry;
			carry = 0;
			while (digit >= 10) {
				carry++;
				digit -= 10;
			}
			sum.at(sum.size() - 1 - j) = digit + ChrDecDiff;
		}
		while (carry > 10) {
			sum.insert(sum.begin(), (carry % 10) + ChrDecDiff);
			carry /= 10;
		}
		if (carry) {
			sum.insert(sum.begin(), carry + ChrDecDiff);
		}
	}
	return sum;
}

size_t sum_bignum_digits(const std::string& number) {
	size_t sum = 0;
	for (size_t i = 0; i < number.size(); i++) {
		sum += static_cast<size_t>(number.at(i) - ChrDecDiff);
	}
	return sum;
}

int main(void) {
	std::cout << sum_bignum_digits(bignum_pow(2, 1000)) << '\n';
}
