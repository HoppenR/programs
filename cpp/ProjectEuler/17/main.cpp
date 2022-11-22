#include <cmath>
#include <iostream>

const size_t numToCountLow[20] = {
    // length of numbers 0 - 19
    4, 3, 3, 5, 4, 4, 3, 5, 5, 4, 3, 6, 6, 8, 8, 7, 7, 9, 8, 8,
};

const size_t numToCountHigh[10] = {
    // length of numbers 0 - 100 in increments of 10
    4, 3, 6, 6, 5, 5, 5, 7, 6, 6,
};

size_t getn(const size_t number, const int n) {
    // returns the nth digit in number, counted from right hand side, 0-based
    return number / static_cast<size_t>(std::pow(10, n)) % 10;
}

size_t num_letter_counts(const size_t num) {
    size_t cnt = 0;
    const size_t tens = num % 100;
    const size_t hundreds = getn(num, 2);
    const size_t thousands = getn(num, 3);
    // Thousands
    if (thousands != 0) {
        cnt += numToCountLow[thousands];
        cnt += 8; // std::string("thousand").length();
    }
    // Hundreds
    if (hundreds != 0) {
        cnt += numToCountLow[hundreds];
        cnt += 7; // std::string("hundred").length();
    }
    // Tens
    if (tens != 0) {
        if (cnt > 0) {
            cnt += 3; // std::string("and").length();
        }
        if (tens < 20) {
            cnt += numToCountLow[tens];
        } else {
            const size_t ones = tens % 10;
            cnt += numToCountHigh[tens / 10];
            if (ones != 0) {
                cnt += numToCountLow[ones];
            }
        }
    }
    return cnt;
}

int main(void) {
    size_t sum = 0;
    for (size_t i = 1; i <= 1000; i++) {
        sum += num_letter_counts(i);
    }
    std::cout << sum << '\n';
}
