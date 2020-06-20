#include <array>
#include <iostream>

const std::array<int, 12> daysPerMonth = {
	31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31,
};

enum Months { february = 1 };
enum WeekDays { monday = 0, sunday = 6 };

int count_sundays(const int start, const int stop, int& weekday) {
	int sundays = 0;
	for (int y = start; y <= stop; y++) {
		for (size_t m = 0; m < 12; m++) {
			if (weekday == WeekDays::sunday) {
				sundays++;
			}
			int daysThisMonth = daysPerMonth.at(m);
			if (m == Months::february) {
				if (((y % 4 == 0) && (y % 100 != 0)) || (y % 400 == 0)) {
					daysThisMonth++;
				}
			}
			weekday = (weekday + daysThisMonth) % 7;
		}
	}
	return sundays;
}

int main(void) {
	int weekday = WeekDays::monday;
	count_sundays(1900, 1900, weekday); // advance weekday
	std::cout << count_sundays(1901, 2000, weekday) << '\n';
}
