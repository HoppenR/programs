#include "../common/IntCode.hpp"
#include <time.h>
#define PART1 1
#define PART2 1

int run_with_input(std::vector<long> prgState, int input) {
	TuringMachine IntCodeComp(prgState);
	IntCodeComp.inputValues.push_back(input);
	while (IntCodeComp.message == 0)
		IntCodeComp.run_program();
	return IntCodeComp.message;
}

int main(void) {
	const time_t start = clock();
	std::vector<long> prgState;
	for (std::string opcode_str; getline(std::cin, opcode_str, ',');)
		prgState.push_back(std::stol(opcode_str));
#if PART1
	std::cout << "p1: " << run_with_input(prgState, 1) << '\n';
#endif // PART1
#if PART2
	std::cout << "p2: " << run_with_input(prgState, 5) << '\n';
#endif // PART2
	const time_t end = clock();
	std::cout << "time: " << difftime(end, start) / CLOCKS_PER_SEC << "s\n";
	return EXIT_SUCCESS;
}
