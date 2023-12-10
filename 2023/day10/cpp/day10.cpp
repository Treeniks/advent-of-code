#include "day10_part1.hpp"
#include "day10_part2.hpp"
#include <iostream>
#include <sstream>

int main() {
    // read from stdin
    std::stringstream buffer;
    buffer << std::cin.rdbuf();
    const std::string input = buffer.str();

    const size_t result_part1 = part1(input);
    std::cout << "Part 1: " << result_part1 << std::endl;

    const size_t result_part2 = part2(input);
    std::cout << "Part 2: " << result_part2 << std::endl;

    return 0;
}
