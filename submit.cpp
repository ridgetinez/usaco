#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <map>
#include <vector>
#include <stdexcept>

class NotLast {
public:
    static void solve(std::istream& input, std::ostream& output) {
        std::string line;

        // Read first line (number of entries) - equivalent to let _: usize = ...
        std::getline(input, line);
        int n = std::stoi(line);

        // Build milk_volumes map - equivalent to the Rust BTreeMap fold operation
        std::map<std::string, int> milk_volumes;

        while (std::getline(input, line)) {
            std::istringstream iss(line);
            std::string name;
            int volume;
            iss >> name >> volume;

            // Equivalent to acc.entry(name).and_modify(|v| *v += volume).or_insert(volume)
            if (milk_volumes.find(name) != milk_volumes.end()) {
                milk_volumes[name] += volume;
            } else {
                milk_volumes[name] = volume;
            }
        }

        // Check if all 7 cows are producing
        bool all_cows_producing = milk_volumes.size() == 7;

        // Create volume_mapping - equivalent to the second fold operation
        std::map<int, std::vector<std::string>> volume_mapping;

        for (const auto& pair : milk_volumes) {
            const std::string& name = pair.first;
            int volume = pair.second;

            // Equivalent to acc.entry(volume).and_modify(...).or_insert(...)
            if (volume_mapping.find(volume) != volume_mapping.end()) {
                volume_mapping[volume].push_back(name);
            } else {
                volume_mapping[volume] = std::vector<std::string>{name};
            }
        }

        // Get maybe_second_last_producers - equivalent to the nth() logic returning Option
        std::map<int, std::vector<std::string>>::iterator maybe_second_last_producers;
        bool has_valid_result = false;

        if (all_cows_producing) {
            // Equivalent to volume_mapping.iter().nth(1)
            auto it = volume_mapping.begin();
            if (volume_mapping.size() > 1) {
                ++it; // Move to second element (index 1)
                maybe_second_last_producers = it;
                has_valid_result = true;
            }
        } else {
            // Equivalent to volume_mapping.iter().nth(0)
            if (!volume_mapping.empty()) {
                maybe_second_last_producers = volume_mapping.begin();
                has_valid_result = true;
            }
        }

        // Output - equivalent to the write_fmt format_args logic with Option handling
        if (!has_valid_result || maybe_second_last_producers->second.size() > 1) {
            output << "Tie\n";
        } else {
            output << maybe_second_last_producers->second[0] << "\n";
        }
    }

    static void run_problem() {
        const std::string input_source = "notlast.in";
        const std::string output_source = "notlast.out";

        // Setup input stream
        std::istream* input_ptr = &std::cin;
        std::ifstream input_file;

        if (input_source != "stdin") {
            input_file.open(input_source);
            if (!input_file) {
                throw std::runtime_error("Failed to open input file");
            }
            input_ptr = &input_file;
        }

        // Setup output stream
        std::ostream* output_ptr = &std::cout;
        std::ofstream output_file;

        if (output_source != "stdout") {
            output_file.open(output_source);
            if (!output_file) {
                throw std::runtime_error("Failed to create output file");
            }
            output_ptr = &output_file;
        }

        solve(*input_ptr, *output_ptr);
    }
};

int main() {
    NotLast::run_problem();
    return 0;
}
