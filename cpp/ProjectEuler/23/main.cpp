#include <algorithm> // permutation
#include <iostream>
#include <numeric> // accumulate
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

bool is_abundant(const long number) {
    const std::set<long> divisors = factors_to_divisors(factorize(number));
    return number < (std::accumulate(divisors.begin(), divisors.end(), 0l));
}

std::set<long> get_abundant_sums(const int lower, const int upper) {
    std::vector<long> abundants;
    std::set<long> abundantSums;
    for (long i = lower; i <= upper; i++) {
        if (!is_abundant(i)) {
            continue;
        }
        abundants.push_back(i);
        for (const long a : abundants) {
            const long curSum = i + a;
            if (curSum <= upper) {
                abundantSums.insert(curSum);
            }
        }
    }
    return abundantSums;
}

constexpr long sum_1_to_n(const long number) {
    return number * (number + 1) / 2;
}

int main(void) {
    const std::set<long> abundantSums = get_abundant_sums(12, 20161);
    constexpr long allnums = sum_1_to_n(20161);
    const long sum = std::accumulate(abundantSums.begin(), abundantSums.end(),
                                     allnums, std::minus<long>{});
    std::cout << sum << '\n';
}
