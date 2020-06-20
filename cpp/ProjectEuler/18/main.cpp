#include "numbers.hpp"
#include <algorithm>
#include <iostream>

int traverse(const std::array<bool, (triangle.size() - 1)> choices) {
	int sum = 0;
	size_t positionx = 0;
	for (size_t i = 0; i < triangle.size(); i++) {
		sum += triangle.at(i).at(positionx);
		if (i < choices.size()) {
			positionx += choices.at(i);
		}
	}
	return sum;
}

int max_sum_every_route() {
	// We only need to move 14 times to cover a triangle with height 15
	std::array<bool, (triangle.size() - 1)> choices;
	choices.fill(false);
	int maxsum = 0;
	for (size_t i = 0; i <= choices.size(); i++) {
		// 1 iteration for every 'true' to add and 1 extra iteration for last
		// iteration that has all bools set to true
		do {
			maxsum = std::max(maxsum, traverse(choices));
		} while (std::next_permutation(choices.begin(), choices.end()));
		if (i < choices.size()) {
			choices.at(choices.size() - 1 - i) = true;
		}
	}
	return maxsum;
}

int main(void) {
	std::cout << max_sum_every_route() << '\n';
}
