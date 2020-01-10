#include <algorithm>
#include <array>
#include <iostream>

int sum_even_fibonaccis(const int upperbound) {
	enum TermIndices { Left, Right, Next };
	std::array<int, 3> terms = { 0, 1, 2 };
	long ans = 0;
	while (terms[Next] <= upperbound) {
		if (terms[Next] % 2 == 0)
			ans += terms[Next];
		std::rotate(terms.begin(), terms.begin() + 1, terms.end());
		terms[Next] = terms[Left] + terms[Right];
	}
	return ans;
}

int main(void) {
	std::cout << sum_even_fibonaccis(4000000) << '\n';
}
