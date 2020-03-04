#include <cmath>
#include <iostream>
#include <numeric> // std::lcm requires -std=c++17

size_t divisible_range_n(int num) {
	if (num == 0)
		return 0;
	num = std::abs(num);
	size_t ans = 1;
	for (int i = 1; i <= num; i++)
		ans = std::lcm(ans, i);
	return ans;
}

int main(void) {
	std::cout << divisible_range_n(20) << '\n';
}
