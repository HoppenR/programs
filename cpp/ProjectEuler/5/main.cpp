#include <iostream>
#include <stdexcept>

bool is_divisible_by(const int n, const int d) {
	return n % d == 0;
}

int calc(const int num) {
	int i = 1;
	while (i++) {
		bool valid = true;
		for (int j = num; j >= 2; j--) {
			if (!is_divisible_by(i, j)) {
				valid = false;
				break;
			}
		}
		if (valid) {
			return i;
		}
	}
	throw std::logic_error("Unreachable code");
}

int main(void) {
	std::cout << calc(20) << '\n';
}
