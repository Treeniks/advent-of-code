#include "day10_part2.hpp"

#include <algorithm>
#include <optional>
#include <set>
#include <sstream>
#include <string>
#include <tuple>
#include <vector>

struct Point {
    int64_t x;
    int64_t y;

    Point operator+(const Point &other) const {
        return {x + other.x, y + other.y};
    }

    Point operator-(const Point &other) const {
        return {x - other.x, y - other.y};
    }

    bool operator==(const Point &other) const {
        return x == other.x && y == other.y;
    }

    Point &operator+=(const Point &other) {
        this->x += other.x;
        this->y += other.y;
        return *this;
    }

    Point &operator-=(const Point &other) {
        this->x -= other.x;
        this->y -= other.y;
        return *this;
    }

    // only required for std::set<Point> to work
    bool operator<(const Point &other) const {
        return x < other.x || (x == other.x && y < other.y);
    }
};

enum Direction {
    Up,
    Right,
    Down,
    Left,
};

enum InsideDirection {
    ILeft,
    IRight,
};

static Point find_S(const std::vector<std::string> &lines) {
    for (int64_t i = 0; i < lines.size(); i++) {
        for (int64_t j = 0; j < lines[i].size(); j++) {
            if (lines[i][j] == 'S') {
                return Point{j, i};
            }
        }
    }

    // unreachable
    return Point{};
}

static std::tuple<Point, Point>
find_neighbours(const std::vector<std::string> &lines, Point pos) {
    std::optional<Point> first = std::nullopt;
    std::optional<Point> second = std::nullopt;

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
            first = Point{x, y - 1};
        }
    }

    // check below
    if (check_below(current) && y + 1 < lines.size()) {
        if (const char below = lines[y + 1][x]; check_above(below)) {
            if (first.has_value())
                second = Point{x, y + 1};
            else
                first = Point{x, y + 1};
        }
    }

    // check left
    if (check_left(current) && x > 0) {
        if (const char left = lines[y][x - 1]; check_right(left)) {
            if (first.has_value())
                second = Point{x - 1, y};
            else
                first = Point{x - 1, y};
        }
    }

    // check right
    if (check_right(current) && x + 1 < lines[y].size()) {
        if (const char right = lines[y][x + 1]; check_left(right)) {
            if (first.has_value())
                second = Point{x + 1, y};
            else
                first = Point{x + 1, y};
        }
    }

    return std::make_tuple(first.value(), second.value());
}

static std::vector<Point> find_loop(const std::vector<std::string> &lines,
                                    const Point start) {
    auto [first, second] = find_neighbours(lines, start);
    std::vector<Point> loop{start};

    auto prev = start;
    auto current = first;
    while (lines[current.y][current.x] != 'S') {
        loop.push_back(current);

        auto [first, second] = find_neighbours(lines, current);

        if (first == prev) {
            prev = current;
            current = second;
        } else {
            prev = current;
            current = first;
        }
    }

    return loop;
}

// return two direction vectors
// they differ only in corners like
// ..........
// ..V.......
// ..|....|..
// ..L----J..
// ..........
// where the first vector will show 'L' to be "Right" and 'J' to be "Up"
// while the second vector will show 'L' to be "Down" and 'J' to be "Right"
// this is needed to really get all inside tiles tested
static std::tuple<std::vector<Direction>, std::vector<Direction>>
find_dirs(const std::vector<Point> &loop) {
    std::vector<Direction> result1;
    std::vector<Direction> result2;

    for (size_t i = 0; i < loop.size(); i++) {
        const Point current = loop[i];
        const Point next = loop[(i + 1) % loop.size()];
        const Point dir = next - current;
        Direction dir_e;

        if (dir.x > 0)
            dir_e = Right;
        else if (dir.x < 0)
            dir_e = Left;
        else if (dir.y > 0)
            dir_e = Down;
        else if (dir.y < 0)
            dir_e = Up;

        result1.push_back(dir_e);
    }

    for (size_t i = 0; i < loop.size(); i++) {
        Point current = loop[i];
        Point prev = loop[(i - 1 + loop.size()) % loop.size()];
        const Point dir = current - prev;
        Direction dir_e;

        if (dir.x > 0)
            dir_e = Right;
        else if (dir.x < 0)
            dir_e = Left;
        else if (dir.y > 0)
            dir_e = Down;
        else if (dir.y < 0)
            dir_e = Up;

        result2.push_back(dir_e);
    }

    return std::make_tuple(result1, result2);
}

static std::set<Point> find_inside_points(const std::vector<Point> &loop,
                                          const std::vector<Direction> &dirs,
                                          const std::vector<Direction> &dirs2,
                                          InsideDirection inside_direction) {
    std::set<Point> result;

    // difference in x assuming direction Up
    int64_t diff;
    switch (inside_direction) {
    case ILeft:
        diff = -1;
        break;
    case IRight:
        diff = +1;
        break;
    }

    auto add_inner_tiles = [&](const std::vector<Direction> &dirs) {
        for (size_t i = 0; i < loop.size(); i++) {
            Point p = loop[i];
            Direction dir = dirs[i];

            Point direction_to_look{};

            switch (dir) {
            case Up:
                direction_to_look = {diff, 0};
                break;
            case Down:
                direction_to_look = {-diff, 0};
                break;
            case Right:
                direction_to_look = {0, diff};
                break;
            case Left:
                direction_to_look = {0, -diff};
                break;
            }

            p += direction_to_look;
            while (std::find(loop.begin(), loop.end(), p) == loop.end()) {
                result.insert(p);
                p += direction_to_look;
            }
        }
    };

    add_inner_tiles(dirs);
    add_inner_tiles(dirs2);

    return result;
}

size_t part2(const std::string &input) {
    std::stringstream ss{input};
    auto lines = std::vector<std::string>{};

    for (std::string line; std::getline(ss, line, '\n');) {
        lines.push_back(line);
    }

    // add an extra line of ground tiles to the bottom
    lines.emplace_back(lines[0].size(), '.');

    const Point start = find_S(lines);
    const std::vector<Point> loop = find_loop(lines, start);
    const auto [dirs, dirs2] = find_dirs(loop);

    // find out which side of the loop is inside
    // iterate through all the bottom ground tiles we added before

    InsideDirection inside = ILeft;

    for (int64_t x = 0; x < lines[0].size(); x += 1) {
        Point bottom{x, static_cast<int64_t>(lines.size()) - 1};

        // then go up until we find a tile that's part of the loop
        auto it = loop.begin();
        do {
            bottom += Point{0, -1};

            // if we have reached the top of the grid
            // we should try the next x
            if (bottom.y < 0)
                goto outer_loop;

            it = std::find(loop.begin(), loop.end(), bottom);
        } while (it == loop.end());

        // requires a special scope so the goto above is happy
        // holy shit
        {
            const size_t index = std::distance(loop.begin(), it);

            const Direction dir = dirs[index];

            switch (dir) {
            case Left:
                inside = IRight;
                break;
            case Right:
                inside = ILeft;
                break;
            case Up:
            case Down:
                continue;
            }

            // we have found which side is inside
            // so we can break the loop now
            break;
        }
    outer_loop:;
    }

    const std::set<Point> inside_points =
        find_inside_points(loop, dirs, dirs2, inside);

    return inside_points.size();
}
