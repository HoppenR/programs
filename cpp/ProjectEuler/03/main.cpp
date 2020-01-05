#include <iostream>
#include <vector>

std::vector<long> factorize(const long n) {
	bool isRunning = true;
	long curFactor = 3;
	long factorSum = 0;
	long ncpy = n;
	std::vector<long> factors;
	while (ncpy % 2 == 0) {
		ncpy /= 2;
		factors.push_back(2);
		factorSum = (factorSum) ? (factorSum * 2) : 2;
	}
	while (curFactor < n && isRunning) {
		if (ncpy % curFactor == 0) {
			ncpy /= curFactor;
			factors.push_back(curFactor);
			factorSum = (factorSum) ? (factorSum * curFactor) : curFactor;
			if (factorSum == n) {
				isRunning = false;
			}
		} else {
			curFactor += 2;
		}
	}
	if (ncpy != 1) {
		factors.push_back(ncpy);
	}
	return factors;
}

int main(void) {
	std::cout << factorize(600851475143).back() << '\n';
}
