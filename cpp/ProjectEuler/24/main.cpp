#include <algorithm>
#include <iostream>
#include <vector>

std::vector<int> nth_permutation(std::vector<int> digs, const int permuts) {
	for (int i = 0; i < permuts - 1; i++) {
		std::next_permutation(digs.begin(), digs.end());
	}
	return digs;
}

long vector_to_long(const std::vector<int> digs) {
	long sum = 0;
	for (const int d : digs) {
		sum = sum * 10 + d;
	}
	return sum;
}

int main(void) {
	const std::vector<int> digs = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9 };
	std::cout << vector_to_long(nth_permutation(digs, 999999)) << '\n';
}
