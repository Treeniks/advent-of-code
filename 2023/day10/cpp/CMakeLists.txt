cmake_minimum_required(VERSION 3.10)

project(day10)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
# link compile_commands.json into source directory so clangd etc. can find it
file(CREATE_LINK
    "${CMAKE_BINARY_DIR}/compile_commands.json"
    "${CMAKE_SOURCE_DIR}/compile_commands.json"
    SYMBOLIC
)

add_executable(day10 day10.cpp day10_part1.cpp day10_part2.cpp)
add_executable(day10_test day10_test.cpp day10_part1.cpp day10_part2.cpp)

enable_testing()
add_test(NAME day10_test COMMAND day10_test)
