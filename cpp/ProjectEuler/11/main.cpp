#include <fstream>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

using Vec2d = std::vector<std::vector<int>>;

long adj_product_dir(const Vec2d& input, const int lStart, const int cStart,
					 const int lDirection, const int cDirection,
					 const int nFactors) {
	long sum = 1;
	try {
		for (int i = 0; i < nFactors; i++) {
			size_t l = static_cast<size_t>(lStart + i * lDirection);
			size_t c = static_cast<size_t>(cStart + i * cDirection);
			sum *= input.at(l).at(c);
		}
	} catch (std::out_of_range& e) {
		return 0;
	}
	return sum;
}

long max_product_n_adj(const Vec2d& input, const size_t nFactors) {
	long largestSum = 0;
	const std::vector<std::pair<int, int>> directions = {
		{ 1, 0 },  // down
		{ 1, 1 },  // down-right
		{ 1, -1 }, // down-left
		{ 0, 1 },  // right
	};
	for (size_t l = 0; l < input.size(); l++) {
		for (size_t c = 0; c < input.at(l).size(); c++) {
			for (auto [lDir, cDir] : directions) {
				long sum = adj_product_dir(input, l, c, lDir, cDir, nFactors);
				largestSum = std::max(largestSum, sum);
			}
		}
	}
	return largestSum;
}

Vec2d file_data_vec2d(const std::string& filename) {
	std::ifstream ifsnum(filename);
	std::stringstream ssif;
	if (ifsnum.is_open()) {
		ssif << ifsnum.rdbuf();
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
