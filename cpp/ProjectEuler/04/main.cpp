#include <cmath>
#include <iostream>
#include <vector>

// NOTE: Can only calculate Largest Palindrome Product for numdigits between 2
// and 9 since we use integers and not strings, and there is no integer type
// that can hold a large enough number for the 10-digit answer.

long getn(const long number, const int n) {
	// returns the nth digit in number, counted from right hand side, 0-based
	return number / static_cast<long>(std::pow(10, n)) % 10;
}

std::vector<int> get_digits(const long number) {
	size_t len = log10(number) + 1;
	std::vector<int> Digits(len);
	for (size_t i = 0; i < len; i++) {
		Digits.at(i) = getn(number, len - 1 - i);
	}
	return Digits;
}

long find_largest_palindromic_number(const int numdigits) {
	if (numdigits <= 1 || numdigits >= 10)
		return -1;
	long largestPalindrome = 0;
	const long start = std::pow(10, numdigits) - 1;
	long end = 0;
	// XXX: We assume that both factors always start with at least
	//      (numdigits / 2) number of nines as their Most Significant Numbers
	for (long i = 0; i < numdigits / 2; i++) {
		end += 9 * std::pow(10, numdigits - 1 - i);
	}
	for (long i = start; i > end; i--) {
		// Decimal palindromic numbers with an even number of digits are
		// divisible by 11.
		// So we can loop from (start / 11) to (end / 11)
		// and the answer will be (i * j * 11) for some i and j in our ranges
		for (long j = start / 11; j > end / 11; j--) {
			const long product = i * j * 11;
			// Check if we can skip iteration
			if (product < largestPalindrome)
				continue;
			std::vector<int> Digits = get_digits(product);
			bool valid = true;
			for (size_t ii = 0; ii < Digits.size() / 2; ii++) {
				if (Digits.at(ii) != Digits.at(Digits.size() - 1 - ii)) {
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
