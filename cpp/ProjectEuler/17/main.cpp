#include <cmath>
#include <iostream>
#include <map>
#include <string>

// XXX: WIP, only works for numbers in the range {1..9999}

const std::map<int, std::string> numToWord = {
	{ 1, "one" },		{ 2, "two" },		  { 3, "three" },
	{ 4, "four" },		{ 5, "five" },		  { 6, "six" },
	{ 7, "seven" },		{ 8, "eight" },		  { 9, "nine" },
	{ 10, "ten" },		{ 11, "eleven" },	  { 12, "twelve" },
	{ 13, "thirteen" }, { 14, "fourteen" },	  { 15, "fifteen" },
	{ 16, "sixteen" },	{ 17, "seventeen" },  { 18, "eighteen" },
	{ 19, "nineteen" }, { 20, "twenty" },	  { 30, "thirty" },
	{ 40, "forty" },	{ 50, "fifty" },	  { 60, "sixty" },
	{ 70, "seventy" },	{ 80, "eighty" },	  { 90, "ninety" },
	{ 100, "hundred" }, { 1000, "thousand" },
};

// const std::map<int, int> numToCount = {
// 	{ 1, 3 },  { 2, 3 },  { 3, 5 },	 { 4, 4 },	 { 5, 4 },	  { 6, 3 },
// 	{ 7, 5 },  { 8, 5 },  { 9, 4 },	 { 10, 3 },	 { 11, 6 },	  { 12, 6 },
// 	{ 13, 8 }, { 14, 8 }, { 15, 7 }, { 16, 7 },	 { 17, 9 },	  { 18, 8 },
// 	{ 19, 8 }, { 20, 6 }, { 30, 6 }, { 40, 5 },	 { 50, 5 },	  { 60, 5 },
// 	{ 70, 7 }, { 80, 6 }, { 90, 6 }, { 100, 7 }, { 1000, 8 },
// };

int getn(const long number, const int n) {
	// returns the nth digit in number, counted from right hand side, 0-based
	return number / static_cast<int>(std::pow(10, n)) % 10;
}

int num_letter_counts(const int num) {
	int cnt = 0;
	const int tens = num % 100;
	const int hundreds = getn(num, 2);
	const int thousands = getn(num, 3);
	// Thousands
	if (thousands != 0) {
		cnt += numToWord.at(thousands).length();
		cnt += numToWord.at(1000).length();
	}
	// Hundreds
	if (hundreds != 0) {
		cnt += numToWord.at(hundreds).length();
		cnt += numToWord.at(100).length();
	}
	// Tens
	if (tens != 0) {
		if (cnt > 0) {
			cnt += std::string("and").length();
		}
		if (tens <= 20) {
			cnt += numToWord.at(tens).length();
		} else {
			const int ones = tens % 10;
			cnt += numToWord.at(tens - ones).length();
			if (ones != 0) {
				cnt += numToWord.at(ones).length();
			}
		}
	}
	return cnt;
}

int main(void) {
	long sum = 0;
	for (int i = 1; i <= 1000; i++) {
		sum += num_letter_counts(i);
	}
	std::cout << sum << '\n';
}
