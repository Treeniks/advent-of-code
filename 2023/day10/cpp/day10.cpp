#include "day10_part1.hpp"
#include "day10_part2.hpp"
#include <iostream>
#include <sstream>

int main() {
    // read from stdin
    std::stringstream buffer;
    buffer << std::cin.rdbuf();
    const std::string input = buffer.str();

    std::cout << "Part 1: " << part1(input) << std::endl;
    std::cout << "Part 2: " << part2(input) << std::endl;

    return 0;
}
