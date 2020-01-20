#include <iostream>

// TODO: optimize this

bool is_prime(int num) {
	for (int i = 2; i < num; i++) {
		if (num % i == 0) {
			return false;
		}
	}
	return true;
}

int find_nth_prime(int num) {
	int primesfound = 0;
	int ans = 0;
	while (primesfound <= num) {
		if (is_prime(++ans)) {
			primesfound++;
		}
	}
	return ans;
}

int main(void) {
	std::cout << find_nth_prime(10001) << '\n';
}
