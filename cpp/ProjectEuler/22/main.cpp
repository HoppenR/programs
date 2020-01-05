#include <algorithm>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

int sum_name_score(const std::vector<std::string>& names) {
	int score = 0;
	for (size_t i = 0; i < names.size(); i++) {
		int namescore = 0;
		for (char c : names.at(i)) {
			namescore += (c - 64);
		}
		namescore *= (i + 1);
		score += namescore;
	}
	return score;
}

std::vector<std::string> file_data_vec(const std::string filename) {
	std::ifstream ifsname(filename);
	std::vector<std::string> names;
	for (std::string line; std::getline(ifsname, line, ',');) {
		line.erase(0, 1); // remove leading "
		line.pop_back();  // remove trailing "
		names.push_back(line);
	}
	names.back().pop_back(); // Remove newline
	std::sort(names.begin(), names.end());
	return names;
}

int main(void) {
	std::cout << sum_name_score(file_data_vec("./p022_names.txt")) << '\n';
}
