#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <map>
#include <sstream>
#include <utility>

namespace citystate {

    size_t n_special_state_pairs_made(
        const std::map<std::pair<std::string, std::string>, size_t>& state_to_city_codes,
        const std::string& state_code,
        const std::string& city_code
    ) {
        if (state_code != city_code) {
            auto it = state_to_city_codes.find(std::make_pair(city_code, state_code));
            if (it != state_to_city_codes.end()) {
                size_t found = it->second;
                return found;
            } else {
                return 0;
            }
        } else {
            return 0;
        }
    }

    void solve(std::istream& input, std::ostream& output) {
        std::string line;
        std::getline(input, line);
        size_t n = std::stoul(line);

        std::vector<std::pair<std::string, std::string>> pairs;
        std::map<std::pair<std::string, std::string>, size_t> occurrences;

        while (std::getline(input, line)) {
            std::istringstream iss(line);
            std::string city, state_code;
            iss >> city >> state_code;

            std::string city_code = city.substr(0, 2);

            auto key = std::make_pair(state_code, city_code);
            occurrences[key]++;

            pairs.push_back(std::make_pair(state_code, city_code));
        }

        size_t special_pair_count = 0;
        for (const auto& pair : pairs) {
            const std::string& state_code = pair.first;
            const std::string& city_code = pair.second;
            special_pair_count += n_special_state_pairs_made(occurrences, state_code, city_code);
        }
        special_pair_count /= 2;

        output << special_pair_count << "\n";
    }

    void run_problem() {
        const std::string input_source = "citystate.in";
        const std::string output_source = "citystate.out";

        std::istream* input_stream = &std::cin;
        std::ifstream input_file;
        if (input_source != "stdin") {
            input_file.open(input_source);
            input_stream = &input_file;
        }

        std::ostream* output_stream = &std::cout;
        std::ofstream output_file;
        if (output_source != "stdout") {
            output_file.open(output_source);
            output_stream = &output_file;
        }

        solve(*input_stream, *output_stream);
    }
}

int main() {
    citystate::run_problem();
    return 0;
}
