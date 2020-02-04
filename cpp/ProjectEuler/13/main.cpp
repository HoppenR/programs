#include <fstream>
#include <gmpxx.h>
#include <iostream>
#include <string>
#include <vector>

// TODO: gmplib is kind of cheating, do this without gmplib

using MpzVector = std::vector<mpz_class>;

size_t sum_bignum(const MpzVector& numbers, const size_t numdigits) {
	mpz_class sum;
	for (const mpz_class& n : numbers) {
		sum += n;
	}
	return std::stoul(sum.get_str().substr(0, numdigits));
}

MpzVector file_data_mpz(const std::string& filename) {
	std::ifstream ifsnum(filename);
	MpzVector numbers;
	if (ifsnum.is_open()) {
		for (std::string line; std::getline(ifsnum, line);) {
			numbers.push_back(mpz_class(line));
		}
	} else {
		throw std::runtime_error("File does not exist");
	}
	return numbers;
}

int main(void) {
	std::cout << sum_bignum(file_data_mpz("numbers.txt"), 10) << '\n';
}
