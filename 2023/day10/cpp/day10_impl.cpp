#include "day10_impl.hpp"

#include <algorithm>
#include <iostream>
#include <numeric>
#include <sstream>
#include <vector>

struct Point {
    size_t x;
    size_t y;

    Point operator+(const Point other) const {
        return {x + other.x, y + other.y};
    }

    Point operator-(const Point other) const {
        return {x - other.x, y - other.y};
    }

    bool operator==(const Point other) const {
        return x == other.x && y == other.y;
    }

    Point operator+=(const Point other) {
        x += other.x;
        y += other.y;
        return *this;
    }

    Point operator-=(const Point other) {
        x -= other.x;
        y -= other.y;
        return *this;
    }
};

struct Grid {
    std::vector<std::string> lines;
    size_t rows;
    size_t columns;

    explicit Grid(std::vector<std::string> lines)
        : lines(lines), rows(lines.size()), columns(lines[0].size()) {}

    char operator[](Point p) const { return lines[p.y][p.x]; }
    char& operator[](Point p) { return lines[p.y][p.x]; }
};

struct Loop {
    std::vector<Point> points;
    std::vector<char> labels;

    [[nodiscard]] size_t size() const { return points.size(); }
};

std::vector<std::string> convert_to_lines(const std::string& input) {
    std::istringstream ss{input};
    std::vector<std::string> lines;
    for (std::string line; std::getline(ss, line);) {
        if (!line.empty()) lines.push_back(line);
    }

    return std::move(lines);
}

Point find_start(const Grid& grid) {
    for (size_t y = 0; y < grid.rows; ++y) {
        for (size_t x = 0; x < grid.columns; ++x) {
            if (grid[{x, y}] == 'S') return {x, y};
        }
    }

    throw std::runtime_error("no S found");
}

std::pair<Point, Point> find_neighbours(const Grid& grid, Point pos) {
    const char c = grid[pos];
    Point above = {pos.x, pos.y - 1};
    Point below = {pos.x, pos.y + 1};
    Point left = {pos.x - 1, pos.y};
    Point right = {pos.x + 1, pos.y};

    switch (c) {
        case '|':
            return std::make_pair(above, below);
        case '-':
            return std::make_pair(left, right);
        case 'J':
            return std::make_pair(above, left);
        case 'L':
            return std::make_pair(above, right);
        case '7':
            return std::make_pair(below, left);
        case 'F':
            return std::make_pair(below, right);
        case 'S': {
            Point first, second;
            bool first_set = false;
            if (grid[above] == '|' || grid[above] == '7' ||
                grid[above] == 'F') {
                first = above;
                first_set = true;
            }
            if (grid[below] == '|' || grid[below] == 'J' ||
                grid[below] == 'L') {
                (first_set ? second : first) = below;
                first_set = true;
            }
            if (grid[left] == '-' || grid[left] == 'L' || grid[left] == 'F') {
                (first_set ? second : first) = left;
                first_set = true;
            }
            if (grid[right] == '-' || grid[right] == '7' ||
                grid[right] == 'J') {
                (first_set ? second : first) = right;
            }
            return std::make_pair(first, second);
        }
        default:
            throw std::runtime_error("cannot find neighbours of ground");
    }
}

Loop find_loop(const Grid& grid, Point start) {
    auto [first, second] = find_neighbours(grid, start);
    Loop loop{{start}, {'S'}};

    Point prev = start;
    Point current = first;
    while (grid[current] != 'S') {
        loop.points.push_back(current);
        loop.labels.push_back(grid[current]);

        auto [first, second] = find_neighbours(grid, current);

        if (first == prev) {
            prev = current;
            current = second;
        } else {
            prev = current;
            current = first;
        }
    }

    return std::move(loop);
}

void convert_start(Grid& grid, Point start) {
    auto [first, second] = find_neighbours(grid, start);

    Point above = {start.x, start.y - 1};
    Point below = {start.x, start.y + 1};
    Point left = {start.x - 1, start.y};
    Point right = {start.x + 1, start.y};

    if ((first == above && second == below) ||
        (first == below && second == above))
        grid[start] = '|';
    else if ((first == left && second == right) ||
             (first == right && second == left))
        grid[start] = '-';
    else if ((first == above && second == left) ||
             (first == left && second == above))
        grid[start] = 'J';
    else if ((first == above && second == right) ||
             (first == right && second == above))
        grid[start] = 'L';
    else if ((first == below && second == left) ||
             (first == left && second == below))
        grid[start] = '7';
    else if ((first == below && second == right) ||
             (first == right && second == below))
        grid[start] = 'F';
}

void convert_grid(Grid& grid, const Loop& loop) {
    // mark all pipes that are part of the loop
    for (Point p : loop.points) {
        grid[p] = '#';
    }

    // replace all other tiles with ground tiles
    for (std::string& line : grid.lines) {
        for (char& c : line) {
            if (c != '#') c = '.';
        }
    }

    // revert loop back to its original pipes
    for (size_t i = 0; i < loop.size(); ++i) {
        grid[loop.points[i]] = loop.labels[i];
    }
}

void convert_loop(Grid& grid, Loop& loop) {
    for (size_t i = 0; i < loop.size(); ++i) {
        Point p = loop.points[i];
        char l = loop.labels[i];
        // uses extended ASCII
        // thus result needs to be viewed with DOS encoding
        switch (l) {
            case '|':
                grid[p] = (char)179;
                break;
            case '-':
                grid[p] = (char)196;
                break;
            case 'J':
                grid[p] = (char)217;
                break;
            case '7':
                grid[p] = (char)191;
                break;
            case 'L':
                grid[p] = (char)192;
                break;
            case 'F':
                grid[p] = (char)218;
                break;
            default:
                break;
        }
    }
}

void print_grid(const Grid& grid) {
    for (auto& line : grid.lines) {
        std::cout << line << std::endl;
    }
}

// expects a converted grid
void mark_inner_outer_tiles(Grid& grid) {
    for (std::string& line : grid.lines) {
        bool inside = false;
        char tmp;
        for (char& c : line) {
            switch (c) {
                case '.':
                    c = inside ? 'I' : 'O';
                    break;
                case '|':
                    inside = !inside;
                    break;
                case 'L':
                    tmp = 'L';
                    break;
                case 'F':
                    tmp = 'F';
                    break;
                case 'J':
                    inside = (tmp == 'F') ? !inside : inside;
                    break;
                case '7':
                    inside = (tmp == 'L') ? !inside : inside;
                    break;
                default:
                    break;
            }
        }
    }
}

size_t count_inner_tiles(const Grid& grid) {
    return std::accumulate(grid.lines.begin(), grid.lines.end(), 0,
                           [](size_t acc, const std::string& line) {
                               return acc +
                                      std::count(line.begin(), line.end(), 'I');
                           });
}

std::pair<size_t, size_t> solve(const std::string& input, bool print) {
    auto lines = convert_to_lines(input);
    Grid grid{lines};

    Point start = find_start(grid);
    auto loop = find_loop(grid, start);

    size_t part1 = loop.size() / 2;

    convert_grid(grid, loop);
    convert_start(grid, start);

    mark_inner_outer_tiles(grid);

    size_t part2 = count_inner_tiles(grid);

    if (print) {
        // convert_loop(grid, loop);
        print_grid(grid);
    }

    return std::make_pair(part1, part2);
}

std::pair<size_t, size_t> solve(const std::string& input) {
    return solve(input, false);
}

size_t part1(const std::string& input) {
    auto [part1, part2] = solve(input);
    return part1;
}

size_t part2(const std::string& input) {
    auto [part1, part2] = solve(input);
    return part2;
}
