#include <iostream>
#include <math.h>
#include <vector>

std::vector<int> get_digits(const long number) {
	std::vector<int> Digits;
	for (int i = log10(number); i >= 0; i--) {
		Digits.push_back(number / static_cast<long>(pow(10, i)) % 10);
	}
	return Digits;
}

long find_largest_palindromic_number(const int numdigits) {
	long largestPalindrome = 0;
	const long target = pow(10, numdigits);
	for (long i = pow(10, numdigits - 1) * 9; i < target; i++) {
		for (long j = pow(10, numdigits - 1) * 9; j < target; j++) {
			const long product = i * j;
			// Check if we can skip iteration
			if (product < largestPalindrome)
				continue;
			// Decimal palindromic numbers with an even number of digits are
			// divisible by 11.
			if (product % 11 != 0)
				continue;
			std::vector<int> Digits = get_digits(product);
			bool valid = true;
			for (size_t ii = 0; ii < Digits.size() / 2; ii++) {
				if (Digits.at(ii) != Digits.at(Digits.size() - ii - 1)) {
					valid = false;
					break;
				}
			}
			if (valid) {
				largestPalindrome = product;
			}
		}
	}
	return largestPalindrome;
}

int main(void) {
	std::cout << find_largest_palindromic_number(3) << '\n';
}
