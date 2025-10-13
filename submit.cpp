#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>
#include <optional>

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
}

namespace breedflip {
    void solve(std::istream& input, std::ostream& output) {
        std::string line;
        std::getline(input, line); // Skip first line

        std::string original;
        std::getline(input, original);

        std::string mutated;
        std::getline(input, mutated);

        int nflips = 0;
        int ndifferent = 0;

        for (size_t i = 0; i < original.length() && i < mutated.length(); ++i) {
            char co = original[i];
            char cm = mutated[i];

            if (co == cm && ndifferent > 0) {
                nflips += 1;
                ndifferent = 0;
            } else if (co != cm) {
                ndifferent += 1;
            }
        }

        if (ndifferent > 0) {
            nflips += 1;
        }

        output << nflips << "\n";
    }

    void run_problem() {
        const std::string input_source = "breedflip.in";
        const std::string output_source = "breedflip.out";

        std::istream* input_ptr = &std::cin;
        std::ifstream input_file;

        if (input_source != "stdin") {
            input_file.open(input_source);
            input_ptr = &input_file;
        }

        std::ostream* output_ptr = &std::cout;
        std::ofstream output_file;

        if (output_source != "stdout") {
            output_file.open(output_source);
            output_ptr = &output_file;
        }

        solve(*input_ptr, *output_ptr);
    }
}

int main() {
    breedflip::run_problem();
    return 0;
}
