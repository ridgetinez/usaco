#include <iostream>
#include <unordered_map>
#include <vector>
#include <string>
#include <sstream>
#include <fstream>
#include <algorithm>
#include <cmath>

namespace yearofthecow {
    const std::vector<std::string> ANIMALS = {
        "Ox", "Tiger", "Rabbit", "Dragon", "Snake", "Horse",
        "Goat", "Monkey", "Rooster", "Dog", "Pig", "Rat"
    };

    long long age_difference(
        const std::string& parent,
        const std::string& child,
        long long direction,
        const std::unordered_map<std::string, std::string>& animal_map
    ) {
        auto parent_animal = animal_map.at(parent);
        auto child_animal = animal_map.at(child);

        long long starting_index = std::find(ANIMALS.begin(), ANIMALS.end(), parent_animal) - ANIMALS.begin();
        long long destination_index = std::find(ANIMALS.begin(), ANIMALS.end(), child_animal) - ANIMALS.begin();

        long long shift = direction;
        long long animals_len = static_cast<long long>(ANIMALS.size());

        while (((starting_index + shift) % animals_len + animals_len) % animals_len != destination_index) {
            shift += direction;
        }

        return shift;
    }

    void dfs(
        const std::unordered_map<std::string, std::string>& animal_map,
        std::unordered_map<std::string, long long>& ages,
        const std::unordered_map<std::string, std::vector<std::pair<long long, std::string>>>& tree,
        const std::string& start
    ) {
        long long starting_age_delta = ages[start];

        auto tree_it = tree.find(start);
        if (tree_it != tree.end()) {
            for (const auto& [direction, child] : tree_it->second) {
                ages[child] = starting_age_delta + age_difference(start, child, direction, animal_map);
                dfs(animal_map, ages, tree, child);
            }
        }
    }

    int alternative_solve(std::istream& input, std::ostream& output) {
        std::string line;
        std::getline(input, line);
        int n = std::stoi(line);

        std::unordered_map<std::string, long long> ages = {{"Bessie", 0}};
        std::unordered_map<std::string, std::string> animals = {{"Bessie", "Ox"}};

        while (std::getline(input, line)) {
            std::istringstream iss(line);
            std::vector<std::string> tokens;
            std::string token;

            while (iss >> token) {
                tokens.push_back(token);
            }

            std::string child = tokens[0];
            std::string parent = tokens.back();
            animals[child] = tokens[4];

            long long direction = (tokens[3] == "next") ? 1 : -1;
            long long starting_age_delta = ages[parent];

            ages[child] = starting_age_delta + age_difference(parent, child, direction, animals);
        }

        output << std::abs(ages["Elsie"]) << std::endl;
        return 0;
    }

    int solve(std::istream& input, std::ostream& output) {
        return alternative_solve(input, output);
    }

    void run_problem() {
        std::string input_source = "stdin";
        std::string output_source = "stdout";

        if (input_source != "stdin") {
            std::ifstream input_file(input_source);
            std::ofstream output_file(output_source);
            solve(input_file, output_file);
        } else {
            solve(std::cin, std::cout);
        }
    }
}

int main() {
    yearofthecow::run_problem();
    return 0;
}
