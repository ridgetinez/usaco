#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>
#include <algorithm>

namespace util {
    namespace parsing {
        template<typename T>
        std::vector<T> parse_line(std::istream& input) {
            std::vector<T> result;
            std::string line;
            if (std::getline(input, line)) {
                std::istringstream iss(line);
                T value;
                while (iss >> value) {
                    result.push_back(value);
                }
            }
            return result;
        }
    }

    namespace printing {
        template<typename T>
        void print_grid(const std::vector<std::vector<T>>& grid) {
            for (const auto& row : grid) {
                for (const auto& elem : row) {
                    std::cout << elem << " ";
                }
                std::cout << "\n";
            }
        }
    }
}

namespace evenmoreodd {
    void solve(std::istream& input, std::ostream& output) {
        std::string line;
        std::getline(input, line); // Skip first line

        std::vector<size_t> ids = util::parsing::parse_line<size_t>(input);

        std::vector<size_t> evens;
        std::vector<size_t> odds;

        for (size_t n : ids) {
            if (n % 2 == 0) {
                evens.push_back(n);
            } else {
                odds.push_back(n);
            }
        }

        size_t nevens = evens.size();
        size_t nodds = odds.size();

        if (nevens >= nodds) {
            output << (2 * nodds) + (nevens > nodds ? 1 : 0) << "\n";
        } else {
            size_t remaining_odds = nodds - nevens;
            size_t result = 2 * nevens;

            switch (remaining_odds % 3) {
                case 0:
                    result += 2 * (remaining_odds / 3);
                    break;
                case 1:
                    result += 2 * (remaining_odds / 3) - 1;
                    break;
                case 2:
                    result += 2 * (remaining_odds / 3) + 1;
                    break;
                default:
                    throw std::runtime_error("wtf");
            }

            output << result << "\n";
        }
    }

    void run_problem() {
        const char* input_source = "stdin";
        const char* output_source = "stdout";

        std::istream* input = &std::cin;
        std::ostream* output = &std::cout;

        std::ifstream input_file;
        std::ofstream output_file;

        if (std::string(input_source) != "stdin") {
            input_file.open(input_source);
            input = &input_file;
        }

        if (std::string(output_source) != "stdout") {
            output_file.open(output_source);
            output = &output_file;
        }

        solve(*input, *output);
    }
}

int main() {
    evenmoreodd::run_problem();
    return 0;
}
