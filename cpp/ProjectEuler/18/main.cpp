#include <algorithm>
#include <array>
#include <iostream>

static const std::array<const std::array<const int, 15>, 15> triangle = { {
	{ 75 },
	{ 95, 64 },
	{ 17, 47, 82 },
	{ 18, 35, 87, 10 },
	{ 20, 4, 82, 47, 65 },
	{ 19, 1, 23, 75, 3, 34 },
	{ 88, 2, 77, 73, 7, 63, 67 },
	{ 99, 65, 4, 28, 6, 16, 70, 92 },
	{ 41, 41, 26, 56, 83, 40, 80, 70, 33 },
	{ 41, 48, 72, 33, 47, 32, 37, 16, 94, 29 },
	{ 53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14 },
	{ 70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57 },
	{ 91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48 },
	{ 63, 66, 4, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31 },
	{ 4, 62, 98, 27, 23, 9, 70, 98, 73, 93, 38, 53, 60, 4, 23 },
} };

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
