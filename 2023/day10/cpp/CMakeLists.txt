cmake_minimum_required(VERSION 3.10)

set(CMAKE_CXX_STANDARD 20)

project(day10)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_executable(day10 day10.cpp day10_impl.cpp)
add_executable(day10_test day10_test.cpp day10_impl.cpp)

enable_testing()
add_test(NAME day10_test COMMAND day10_test)
