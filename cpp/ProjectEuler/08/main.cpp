#include <deque>
#include <fstream>
#include <iostream>
#include <stdexcept>
#include <string>

// NOTE: Very easy to run into integer overflows when using 20+ factors

size_t max_product_n_adj(const std::string& numbers, const size_t nFactors) {
	size_t currentProduct = 0;
	size_t greatestProduct = 0;
	std::deque<int> factors;
	for (size_t i = 0; i < nFactors; i++) {
		factors.push_back(numbers.at(i) - '0');
	}
	for (size_t i = nFactors; i < numbers.size(); i++) {
		currentProduct = 1;
		for (const int f : factors) {
			currentProduct *= static_cast<size_t>(f);
		}
		greatestProduct = std::max(greatestProduct, currentProduct);
		factors.push_back(numbers.at(i) - '0');
		factors.pop_front();
	}
	return greatestProduct;
};

std::string file_data(const std::string& filename) {
	std::ifstream file(filename);
	if (file.is_open()) {
		std::string data;
		while (file.peek() != EOF) {
			char c = file.get();
			if (c != '\n') {
				data += c;
			}
		}
		file.close();
		return data;
	}
	throw std::runtime_error("File does not exist");
}

int main(void) {
	std::cout << max_product_n_adj(file_data("numbers.txt"), 13) << '\n';
}
