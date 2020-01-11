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
	const long start = pow(10, numdigits) - 1;
	// XXX: We assume that both of the factors' Most Significant Digit
	//      is always going to be 9     here ↓
	const long end = pow(10, numdigits - 1) * 9;
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
