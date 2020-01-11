#include <iostream>
#include <math.h>
#include <vector>

std::vector<int> get_digits(long number) {
	std::vector<int> Digits;
	for (int i = log10(number); i >= 0; i--) {
		Digits.push_back(number / static_cast<int>(pow(10, i)) % 10);
	}
	return Digits;
}

long find_largest_palindromic_number(int numdigits) {
	std::vector<int> Digits;
	long largestPalindrome = 0;
	for (long i = pow(10, numdigits - 1) * 9; i < pow(10, numdigits); i++) {
		for (long j = pow(10, numdigits - 1) * 9; j < pow(10, numdigits); j++) {
			long product = i * j;
			// Check if we can skip iteration
			if (product < largestPalindrome)
				continue;
			// Decimal palindromic numbers with an even number of digits are
			// divisible by 11.
			if (product % 11 != 0)
				continue;
			Digits = get_digits(product);
			bool valid = true;
			for (size_t ii = 0; ii < Digits.size(); ii++) {
				if (Digits.at(ii) != Digits.at(Digits.size() - ii - 1)) {
					valid = false;
					break;
				}
			}
			if (valid) {
				largestPalindrome = std::max(largestPalindrome, product);
			}
		}
	}
	return largestPalindrome;
}

int main(void) {
	std::cout << find_largest_palindromic_number(3) << '\n';
}
