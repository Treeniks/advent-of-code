#include <iostream>
#include <sstream>

#include "day10_impl.hpp"

int main() {
    // read from stdin
    std::ostringstream buffer;
    buffer << std::cin.rdbuf();
    const std::string input = buffer.str();

    auto [result_part1, result_part2] = solve(input, true);
    std::cout << "Part 1: " << result_part1 << std::endl;
    std::cout << "Part 2: " << result_part2 << std::endl;

    return 0;
}
