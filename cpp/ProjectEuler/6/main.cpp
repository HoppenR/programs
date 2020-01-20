#include <iostream>
#include <math.h>

long sum_square_difference(long num) {
	num = std::abs(num);
	// My attempt at summing every number from 1 to num,
	// the right hand side of the plus-sign is for when num is odd.
	long sum = ((num / 2) * (num + 1)) + ((num + 1) * (num % 2) / 2);
	long sumSquared = pow(sum, 2);
	long squaresSum = 0;
	for (long i = 1; i <= num; i++) {
		squaresSum += pow(i, 2);
	}
	return sumSquared - squaresSum;
}

int main(void) {
	std::cout << sum_square_difference(100) << '\n';
}
