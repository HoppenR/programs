#include <cmath>
#include <iostream>
#include <stdexcept>

long find_pythagorean_triplet_for_sum(const int sum) {
	// returns the pythagorean triplet whose terms equal sum
	// in the following format: (a * b * c)
	for (int a = 1; a < sum; a++) {
		for (int b = a + 1; b < sum; b++) {
			const long csq = std::pow(a, 2) + std::pow(b, 2);
			const double c = std::sqrt(csq);
			if (a + b + c == sum)
				return a * b * c;
		}
	}
	throw std::runtime_error("No Pythagorean triplet equals this sum");
}

int main(void) {
	std::cout << find_pythagorean_triplet_for_sum(1000) << '\n';
}
