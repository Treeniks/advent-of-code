#include <iostream>

#include "day10_impl.hpp"

// undefine NDEBUG so asserts aren't disabled even in release mode
#undef NDEBUG
#include <cassert>

const std::string example1 = R"(..F7.
.FJ|.
SJ.L7
|F--J
LJ...
)";

const std::string example2 = R"(...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
)";

const std::string example3 = R"(.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
)";

const std::string example4 = R"(FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
)";

void test_part1() { assert(part1(example1) == 8); }
void test_part2_1() { assert(part2(example1) == 1); }
void test_part2_2() { assert(part2(example2) == 4); }
void test_part2_3() { assert(part2(example3) == 8); }
void test_part2_4() { assert(part2(example4) == 10); }

int main() {
    test_part1();

    test_part2_1();
    test_part2_2();
    test_part2_3();
    test_part2_4();

    std::cout << "All tests passed" << std::endl;
    return 0;
}
