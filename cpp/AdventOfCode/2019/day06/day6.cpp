#include <climits>
#include <iostream>
#include <map>
#include <sstream>
#include <string>
#include <time.h>
#include <vector>
#define PART1 1
#define PART2 1

// Requires c++17 to compile because of the usage of structured bindings

typedef std::pair<std::string, std::string> NodeInfo;
// [planetName, parentName]
typedef std::map<std::string, std::string> NodeMap;
// [ [planetName, parentname], ]
typedef std::map<std::string, int> AncestorMap;
// [ [planetName, stepsThere], ]

class P {
public:
	AncestorMap ancestors;
	int steps = 0;
	std::string planet;
	P(std::string planet) {
		this->planet = planet;
	}
};

int count_orbits(const NodeMap& orbitTree) {
	int count = 0;
	for (const NodeInfo& node : orbitTree) {
		std::string planet = node.first;
		while (planet != "COM") {
			count++;
			planet = orbitTree.at(planet);
		}
	}
	return count;
}

int count_steps_intersect_node(const NodeMap& orbitTree) {
	enum peopleindex { YOU = 0, SAN = 1 };
	std::vector<P> People = {
		{ P(orbitTree.at("YOU")) },
		{ P(orbitTree.at("SAN")) },
	};
	for (P& Person : People) {
		while (Person.planet != "COM") {
			Person.ancestors.insert(
				std::make_pair(Person.planet, Person.steps));
			Person.planet = orbitTree.at(Person.planet);
			Person.steps++;
		}
	}
	int leaststeps = INT_MAX;
	for (const auto& [planet, y_steps] : People.at(YOU).ancestors) {
		if (People.at(SAN).ancestors.find(planet) !=
			People.at(SAN).ancestors.end()) {
			const int s_steps = People.at(SAN).ancestors.at(planet);
			const int steps = y_steps + s_steps;
			leaststeps = std::min(leaststeps, steps);
		}
	}
	return leaststeps;
}

int main(void) {
	const time_t start = clock();
	NodeMap orbitTree;
	std::string inputString;
	for (std::string line; std::getline(std::cin, line);) {
		std::stringstream lineStream(line);
		std::string planetName, parentName;
		getline(lineStream, parentName, ')');
		getline(lineStream, planetName);
		orbitTree.insert(NodeInfo(planetName, parentName));
	}
#if PART1
	std::cout << "p1: " << count_orbits(orbitTree) << '\n';
#endif // PART1
#if PART2
	std::cout << "p2: " << count_steps_intersect_node(orbitTree) << '\n';
#endif // PART2
	const time_t end = clock();
	std::cout << "time: " << difftime(end, start) / CLOCKS_PER_SEC << "s\n";
	return EXIT_SUCCESS;
}
