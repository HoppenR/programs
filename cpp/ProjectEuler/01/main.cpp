#include <algorithm>
#include <iostream>
#include <vector>

long sum_fizzbuzz_range(const int upperbound, const std::vector<int>& M) {
	long ans = 0;
	for (int i = 1; i < upperbound; i++)
		if (std::any_of(M.begin(), M.end(), [i](int n) { return i % n == 0; }))
			ans += i;
	return ans;
}

int main(void) {
	std::cout << sum_fizzbuzz_range(1000, { 3, 5 }) << '\n';
}
