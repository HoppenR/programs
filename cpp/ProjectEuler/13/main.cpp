#include <algorithm>
#include <fstream>
#include <iostream>
#include <stdexcept>
#include <string>
#include <vector>

constexpr int ChrDecDiff = 48; // 1 + 48 = '1'

using StrVec = std::vector<std::string>;

bool max_size_cmp(const std::string& s, const std::string& t) {
    return s.size() < t.size();
}

size_t sum_bignum(const StrVec& numbers, const size_t numdigits) {
    const size_t maxlen =
        std::max_element(numbers.begin(), numbers.end(), max_size_cmp)->size();
    std::string sum(maxlen, '0');
    for (const std::string& n : numbers) {
        short carry = 0;
        for (size_t i = 0; i < sum.size(); i++) {
            short digit = sum.at(sum.size() - 1 - i) - ChrDecDiff + carry;
            if (i < n.size()) {
                digit += n.at(n.size() - 1 - i) - ChrDecDiff;
            }
            if (digit >= 10) {
                carry = 1;
                digit -= 10;
            } else {
                carry = 0;
            }
            sum.at(sum.size() - 1 - i) = digit + ChrDecDiff;
        }
        if (carry) {
            sum.insert(sum.begin(), carry + ChrDecDiff);
        }
    }
    return std::stoul(sum.substr(0, numdigits));
}

StrVec file_data_string(const std::string& filename) {
    std::ifstream ifsnum(filename);
    StrVec numbers;
    if (ifsnum.is_open()) {
        for (std::string line; std::getline(ifsnum, line);) {
            numbers.push_back(std::string(line));
        }
    } else {
        throw std::runtime_error("File does not exist");
    }
    return numbers;
}

int main(void) {
    std::cout << sum_bignum(file_data_string("numbers.txt"), 10) << '\n';
}
