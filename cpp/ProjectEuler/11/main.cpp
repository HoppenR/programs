#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

using Vec2d = std::vector<std::vector<int>>;

long max_product_n_adj(const Vec2d& input, const size_t nFactors) {
	long largestSum = 0;
	for (size_t l = 0; l < input.size(); l++) {
		for (size_t c = 0; c < input.at(l).size(); c++) {
			// XXX: This if-block assumes that all lines have the same length
			if (l <= input.size() - nFactors) {
				// Compare vertically down
				{
					long sum = 1;
					for (size_t i = 0; i < nFactors; i++) {
						sum *= input.at(l + i).at(c);
					}
					largestSum = std::max(largestSum, sum);
				}
				// Compare Diagonally down-right
				if (c <= input.at(l).size() - nFactors) {
					long sum = 1;
					for (size_t i = 0; i < nFactors; i++) {
						sum *= input.at(l + i).at(c + i);
					}
					largestSum = std::max(largestSum, sum);
				}
				// Compare Diagonally down-left
				if (c >= 3) {
					long sum = 1;
					for (size_t i = 0; i < nFactors; i++) {
						sum *= input.at(l + i).at(c - i);
					}
					largestSum = std::max(largestSum, sum);
				}
			}
			if (c <= input.at(l).size() - nFactors) {
				// Compare horizontally right
				{
					long sum = 1;
					for (size_t i = 0; i < nFactors; i++) {
						sum *= input.at(l).at(c + i);
					}
					largestSum = std::max(largestSum, sum);
				}
			}
		}
	}
	return largestSum;
}

Vec2d file_data_vec2d(const std::string& filename) {
	std::ifstream ifs(filename, std::ifstream::in);
	std::stringstream ssif;
	if (ifs.is_open()) {
		ssif << ifs.rdbuf();
		ifs.close();
	} else {
		throw std::runtime_error("File does not exist");
	}
	Vec2d inputInts;
	for (size_t i = 0; ssif.peek() != EOF; i++) {
		inputInts.push_back({});
		std::string line;
		std::getline(ssif, line);
		std::stringstream ss(line);
		while (!ss.eof()) {
			int num = 0;
			ss >> num;
			inputInts.at(i).push_back(num);
		}
	}
	return inputInts;
}

int main(void) {
	std::cout << max_product_n_adj(file_data_vec2d("numbers.txt"), 4) << '\n';
}
