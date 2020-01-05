#include <iostream>
#include <map>
#include <utility>
#include <vector>

// TODO: optimize this

std::vector<size_t> factorize(const size_t n) {
	bool isRunning = true;
	size_t curFactor = 3;
	size_t factorPrd = 0;
	size_t ncpy = n;
	std::vector<size_t> factors;
	while (ncpy % 2 == 0) {
		ncpy /= 2;
		factors.push_back(2);
		factorPrd = (factorPrd) ? (factorPrd * 2) : 2;
	}
	while (curFactor < n && isRunning) {
		if (ncpy % curFactor == 0) {
			ncpy /= curFactor;
			factors.push_back(curFactor);
			factorPrd = (factorPrd) ? (factorPrd * curFactor) : curFactor;
			if (factorPrd == n) {
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

// https://www.math.upenn.edu/~deturck/m170/wk2/numdivisors.html
size_t num_factors_for_n(const size_t n) {
	const std::vector<size_t> factors = factorize(n);
	std::map<size_t, size_t> factorMap;
	for (const size_t f : factors) {
		if (!factorMap.insert(std::make_pair(f, 1)).second) {
			factorMap.at(f) += 1;
		}
	}
	size_t numFactors = 1;
	for (const std::pair<size_t, size_t>& kv : factorMap) {
		numFactors *= (kv.second + 1);
	}
	return numFactors;
}

size_t triangle_with_n_factors(const size_t nFactors) {
	size_t maxFactors = 0;
	size_t triangle = 0;
	for (size_t i = 1; maxFactors < nFactors; i++) {
		triangle += i;
		maxFactors = std::max(maxFactors, num_factors_for_n(triangle));
	}
	return triangle;
}

int main(void) {
	std::cout << triangle_with_n_factors(500) << '\n';
}
