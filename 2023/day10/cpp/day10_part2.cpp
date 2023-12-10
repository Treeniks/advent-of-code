#include "day10_part2.hpp"

#include <optional>
#include <sstream>
#include <string>
#include <tuple>
#include <vector>

static std::tuple<size_t, size_t> find_S(std::vector<std::string> &lines) {
    for (size_t i = 0; i < lines.size(); i++) {
        for (size_t j = 0; j < lines[i].size(); j++) {
            if (lines[i][j] == 'S') {
                return std::make_tuple(j, i);
            }
        }
    }

    // unreachable
    return std::make_tuple(0, 0);
}

static std::tuple<std::tuple<size_t, size_t>, std::tuple<size_t, size_t>>
find_neighbours(const std::vector<std::string> &lines,
                std::tuple<size_t, size_t> pos) {
    std::optional<std::tuple<size_t, size_t>> first = std::nullopt;
    std::optional<std::tuple<size_t, size_t>> second = std::nullopt;

    auto [x, y] = pos;
    const char current = lines[y][x];

    auto check_above = [](char c) {
        return c == '|' || c == 'L' || c == 'J' || c == 'S';
    };
    auto check_below = [](char c) {
        return c == '|' || c == '7' || c == 'F' || c == 'S';
    };
    auto check_left = [](char c) {
        return c == '-' || c == 'J' || c == '7' || c == 'S';
    };
    auto check_right = [](char c) {
        return c == '-' || c == 'L' || c == 'F' || c == 'S';
    };

    // check above
    if (check_above(current) && y > 0) {
        if (const char above = lines[y - 1][x]; check_below(above)) {
            first = std::make_tuple(x, y - 1);
        }
    }

    // check below
    if (check_below(current) && y + 1 < lines.size()) {
        if (const char below = lines[y + 1][x]; check_above(below)) {
            if (first.has_value())
                second = std::make_tuple(x, y + 1);
            else
                first = std::make_tuple(x, y + 1);
        }
    }

    // check left
    if (check_left(current) && x > 0) {
        if (const char left = lines[y][x - 1]; check_right(left)) {
            if (first.has_value())
                second = std::make_tuple(x - 1, y);
            else
                first = std::make_tuple(x - 1, y);
        }
    }

    // check right
    if (check_right(current) && x + 1 < lines[y].size()) {
        if (const char right = lines[y][x + 1]; check_left(right)) {
            if (first.has_value())
                second = std::make_tuple(x + 1, y);
            else
                first = std::make_tuple(x + 1, y);
        }
    }

    return std::make_tuple(first.value(), second.value());
}

static std::vector<char> find_loop(const std::vector<std::string> &lines,
                                   std::tuple<size_t, size_t> start) {
    auto [first, second] = find_neighbours(lines, start);

    auto [prev_x, prev_y] = start;
    auto [current_x, current_y] = first;

    std::vector<char> loop{lines[prev_y][prev_x]};
    while (lines[current_y][current_x] != 'S') {
        loop.push_back(lines[current_y][current_x]);

        // std::cout << lines[current_y][current_x] << std::endl;

        auto [first, second] =
            find_neighbours(lines, std::make_tuple(current_x, current_y));
        auto [first_x, first_y] = first;
        auto [second_x, second_y] = second;

        if (first_x == prev_x && first_y == prev_y) {
            prev_x = current_x;
            prev_y = current_y;
            current_x = second_x;
            current_y = second_y;
        } else {
            prev_x = current_x;
            prev_y = current_y;
            current_x = first_x;
            current_y = first_y;
        }
    }

    return loop;
}

size_t part2(const std::string &input) { return 0; }
