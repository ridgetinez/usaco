#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <sstream>
#include <algorithm>

namespace util {
    namespace parsing {
        template<typename T>
        std::vector<T> parse_line(std::istream& input) {
            std::string line;
            std::getline(input, line);
            std::istringstream iss(line);
            std::vector<T> result;
            T value;
            while (iss >> value) {
                result.push_back(value);
            }
            return result;
        }
    }

    namespace printing {
        template<typename T>
        void print_grid(const std::vector<std::vector<T>>& grid) {
            for (const auto& row : grid) {
                std::cout << "[";
                for (size_t i = 0; i < row.size(); ++i) {
                    std::cout << row[i];
                    if (i < row.size() - 1) std::cout << ", ";
                }
                std::cout << "]" << std::endl;
            }
        }
    }
}

namespace outofplace {
    void solve(std::istream& input, std::ostream& output) {
        std::string line;
        std::getline(input, line); // Read and discard first line

        std::vector<size_t> heights;
        while (std::getline(input, line)) {
            heights.push_back(std::stoull(line));
        }

        int nswaps = 0;
        for (int i = heights.size() - 1; i >= 0; --i) {
            // Find max element in range [0, i)
            int swap_index = -1;
            size_t max_val = 0;
            for (int j = i - 1; j >= 0; --j) {
                if (heights[j] >= max_val) {
                    max_val = heights[j];
                    swap_index = j;
                }
            }

            // Only swap if we found an element greater than heights[i]
            if (swap_index != -1 && heights[i] < heights[swap_index]) {
                std::swap(heights[i], heights[swap_index]);
                nswaps++;
            }
        }

        output << nswaps << std::endl;
    }

    void run_problem() {
        const std::string input_source = "outofplace.in";
        const std::string output_source = "outofplace.out";

        std::istream* input = &std::cin;
        std::ifstream input_file;
        if (input_source != "stdin") {
            input_file.open(input_source);
            input = &input_file;
        }

        std::ostream* output = &std::cout;
        std::ofstream output_file;
        if (output_source != "stdout") {
            output_file.open(output_source);
            output = &output_file;
        }

        solve(*input, *output);
    }
}

int main() {
    outofplace::run_problem();
    return 0;
}
