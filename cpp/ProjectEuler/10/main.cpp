#include <algorithm>
#include <cmath>
#include <iostream>
#include <vector>

long sum_primes_range_n(const size_t upperbound) {
	// Sums all primes from 2 to upperbound (exclusive)
	std::vector<bool> numsB(upperbound, true);
	numsB.at(0) = false;
	numsB.at(1) = false;
	const size_t target = std::sqrt(upperbound);
	for (size_t i = 0; i <= target; i++) {
		if (numsB.at(i)) {
			for (size_t j = std::pow(i, 2); j < upperbound; j += i) {
				// Since i is a prime we need to start marking from at least 2i,
				// but we can start at i² for a slight optimization,
				// then we mark every multiple of i after our starting point.
				numsB.at(j) = false;
			}
		}
	}
	int i = 0;
	long sum = 0;
	// sum += bool(0 or 1) * index
	std::for_each(numsB.begin(), numsB.end(), [&](bool a) { sum += a * i++; });
	return sum;
}

int main(void) {
	std::cout << sum_primes_range_n(2000000) << '\n';
}
