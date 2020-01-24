#include <iostream>
#include <math.h>

// TODO: Optimize this

bool is_prime(const int num) {
	const int target = sqrt(num);
	for (int i = 2; i <= target; i++) {
		if (num % i == 0) {
			return false;
		}
	}
	return true;
}

long sum_primes_range_n(const int upperbound) {
	long sum = 2;
	for (int i = 3; i < upperbound; i += 2) {
		if (is_prime(i))
			sum += i;
	}
	return sum;
}

int main(void) {
	std::cout << sum_primes_range_n(2000000) << '\n';
}
