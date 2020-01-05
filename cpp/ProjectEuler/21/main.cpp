#include <algorithm>
#include <iostream>
#include <numeric>
#include <set>
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

std::set<long> factors_to_divisors(const std::vector<long>& factors) {
	std::vector<bool> choices(factors.size(), false);
	std::set<long> divs;
	for (size_t i = 0; i < factors.size(); i++) {
		do {
			long curdiv = 1;
			for (size_t j = 0; j < factors.size(); j++) {
				if (choices.at(j)) {
					curdiv *= factors.at(j);
				}
			}
			divs.insert(curdiv);
		} while (std::next_permutation(choices.begin(), choices.end()));
		choices.at(choices.size() - 1 - i) = true;
	}
	return divs;
}

long sum_set(const std::set<long>& con) {
	return std::accumulate(con.begin(), con.end(), 0);
}

std::set<long> amicables_range_n(const int upperbound) {
	std::set<long> amicables;
	for (int i = 2; i < upperbound; i++) {
		long a = sum_set(factors_to_divisors(factorize(i)));
		if (i == a) {
			continue;
		}
		long b = sum_set(factors_to_divisors(factorize(a)));
		if (i != b) {
			continue;
		}
		amicables.insert(i);
	}
	return amicables;
}

int main(void) {
	std::cout << sum_set(amicables_range_n(10000)) << '\n';
}
