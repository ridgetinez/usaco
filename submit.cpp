#include <iostream>
#include <fstream>
#include <string>
#include <unordered_map>
#include <sstream>

size_t n_special_state_pairs_made(
    std::unordered_map<std::string, std::unordered_map<std::string, size_t>>& state_to_city_codes,
    const std::string& state_code,
    const std::string& city_code
) {
    // Calculate 'a': count from city_code -> state_code mapping
    size_t a = 0;
    auto city_it = state_to_city_codes.find(city_code);
    if (city_it != state_to_city_codes.end()) {
        auto state_it = city_it->second.find(state_code);
        if (state_it != city_it->second.end()) {
            a = state_it->second;
        }
    }

    // Calculate 'b': if state_code == city_code, get count from state_code -> city_code mapping
    size_t b = 0;
    if (state_code == city_code) {
        auto state_it = state_to_city_codes.find(state_code);
        if (state_it != state_to_city_codes.end()) {
            auto city_it_inner = state_it->second.find(city_code);
            if (city_it_inner != state_it->second.end()) {
                b = city_it_inner->second;
            }
        }
    }

    size_t npairs_made = a - b;

    // Increment the count for state_code -> city_code
    state_to_city_codes[state_code][city_code] += 1;

    return npairs_made;
}

int solve(std::istream& input, std::ostream& output) {
    std::string line;

    // Read the first line (number of entries)
    if (!std::getline(input, line)) {
        return -1;
    }
    int n = std::stoi(line);

    size_t n_special_state_pairs = 0;
    std::unordered_map<std::string, std::unordered_map<std::string, size_t>> state_to_city_codes;

    // Process each city-state pair (mimicking the fold operation)
    for (int i = 0; i < n; i++) {
        if (!std::getline(input, line)) {
            break;
        }

        std::istringstream iss(line);
        std::string city, state_code;

        if (!(iss >> city >> state_code)) {
            continue;
        }

        // Extract first two characters of city as city_code
        std::string city_code = city.substr(0, 2);

        n_special_state_pairs += n_special_state_pairs_made(state_to_city_codes, state_code, city_code);
    }

    output << n_special_state_pairs << std::endl;
    return 0;
}

void run_problem() {
    const std::string input_source = "citystate.in";
    const std::string output_source = "citystate.out";

    std::istream* input_ptr = &std::cin;
    std::ifstream input_file;

    if (input_source != "stdin") {
        input_file.open(input_source);
        if (input_file.is_open()) {
            input_ptr = &input_file;
        }
    }

    std::ostream* output_ptr = &std::cout;
    std::ofstream output_file;

    if (output_source != "stdout") {
        output_file.open(output_source);
        if (output_file.is_open()) {
            output_ptr = &output_file;
        }
    }

    solve(*input_ptr, *output_ptr);

    if (input_file.is_open()) {
        input_file.close();
    }
    if (output_file.is_open()) {
        output_file.close();
    }
}

int main() {
    run_problem();
    return 0;
}
