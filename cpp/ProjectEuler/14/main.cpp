#include <algorithm>
#include <iostream>
#include <vector>

// Returns the starting number of the longest collatz sequence between 1 and n
size_t longest_collatz_range_n(const size_t upperbound) {
	std::vector<size_t> seqs(upperbound);
	for (size_t startNum = 2; startNum < upperbound; startNum++) {
		size_t chainlen = 0;
		size_t i = startNum;
		// iterate until we can use a previously calculated chainlen in seqs
		while (i >= startNum) {
			if (i % 2 == 0) {
				i /= 2;
			} else {
				i = i * 3 + 1;
			}
			chainlen++;
		}
		seqs[startNum] = chainlen + seqs[i];
	}
	// Return the index of the longest sequence, this will be the best start num
	long bestStrtNr = std::max_element(seqs.begin(), seqs.end()) - seqs.begin();
	return static_cast<size_t>(bestStrtNr);
}

int main(void) {
	std::cout << longest_collatz_range_n(1000000) << '\n';
}
