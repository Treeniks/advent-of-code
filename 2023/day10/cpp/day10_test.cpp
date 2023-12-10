#include "day10_impl.hpp"
#include <iostream>

// undefine NDEBUG so asserts aren't disabled even in release mode
#undef NDEBUG
#include <cassert>

const std::string example = R"(..F7.
.FJ|.
SJ.L7
|F--J
LJ...
)";

void test_part1() { assert(part1(example) == 8); }

int main() {
    test_part1();

    std::cout << "All tests passed" << std::endl;
    return 0;
}
