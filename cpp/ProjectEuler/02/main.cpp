#include <climits>
#include <deque>
#include <iostream>

size_t sum_even_fibonaccis(const size_t upperbound) {
	std::deque<size_t> terms = { 1, 1 };
	size_t ans = 0;
	while (terms.back() <= upperbound) {
		terms.push_back(terms.at(0) + terms.at(1));
		if (terms.back() % 2 == 0)
			ans += terms.back();
		terms.pop_front();
	}
	return ans;
}

int main(void) {
	std::cout << sum_even_fibonaccis(4000000) << '\n';
}
