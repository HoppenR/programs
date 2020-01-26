#include <iostream>
#include <math.h>
#include <vector>

long sum_primes_range_n(const size_t upperbound) {
	// Sums all primes from 2 to upperbound (exclusive)
	std::vector<bool> numbers(upperbound, true);
	numbers.at(0) = false;
	numbers.at(1) = false;
	const size_t target = sqrt(upperbound);
	for (size_t i = 0; i <= target; i++) {
		if (numbers.at(i)) {
			for (size_t j = pow(i, 2); j < upperbound; j += i) {
				// Since i is a prime we need to start marking from at least 2i,
				// but we can start at i² for a slight optimization,
				// then we mark every multiple of i after our starting point.
				numbers.at(j) = false;
			}
		}
	}
	long sum = 0;
	for (size_t i = 0; i < numbers.size(); i++) {
		if (numbers.at(i)) {
			sum += i;
		}
	}
	return sum;
}

int main(void) {
	std::cout << sum_primes_range_n(2000000) << '\n';
}
