#include <deque>
#include <fstream>
#include <iostream>
#include <string>
#define CHAROFFSET 48

size_t get_largest_product_n_adj(std::string numbers, size_t nAdjacent) {
	size_t currentProduct = 0;
	size_t greatestProduct = 0;
	std::deque<int> factors;
	// Populate the queue before the main loop
	for (size_t i = 0; i < nAdjacent; i++) {
		factors.push_back(numbers.at(i) - CHAROFFSET);
	}
	// Check the product of the queue then add next, and remove the first number
	for (size_t i = nAdjacent; i < numbers.size(); i++) {
		currentProduct = 1;
		for (int f : factors) {
			currentProduct *= static_cast<size_t>(f);
		}
		greatestProduct = std::max(greatestProduct, currentProduct);
		factors.push_back(numbers.at(i) - CHAROFFSET);
		factors.pop_front();
	}
	return greatestProduct;
};

int main(void) {
	std::ifstream file("input");
	if (file.is_open()) {
		std::string data;
		while (file.peek() != EOF) {
			char c = file.get();
			if (c != '\n') {
				data += c;
			}
		}
		file.close();
		std::cout << get_largest_product_n_adj(data, 13) << '\n';
	}
}
