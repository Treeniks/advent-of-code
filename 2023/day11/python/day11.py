import itertools
import sys

example = """...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"""

def no_galaxies(line):
    for c in line:
        if c != '.':
            return False
    return True

def get_column(input, column):
    result = []
    for i in range(0, len(input)):
        result.append(input[i][column])
    return result

def get_galaxies(input):
    result = []
    for row in range(0, len(input)):
        for column in range(0, len(input[0])):
            if input[row][column] == '#':
                result.append((column, row))
    return result

def number_of_empty_rows_between(input, begin, end):
    if begin > end:
        tmp = begin
        begin = end
        end = tmp
    result = 0
    for row in range(begin, end):
        if no_galaxies(input[row]):
            result += 1
    return result

def number_of_empty_columns_between(input, begin, end):
    if begin > end:
        tmp = begin
        begin = end
        end = tmp
    result = 0
    for column in range(begin, end):
        if no_galaxies(get_column(input, column)):
            result += 1
    return result

def solve(input, multiplier):
    galaxies = get_galaxies(input)

    distances = []

    for ((x1, y1), (x2, y2)) in itertools.combinations(galaxies, 2):
        num1 = number_of_empty_rows_between(input, y1, y2)
        num2 = number_of_empty_columns_between(input, x1, x2)
        distance = abs(x2 - x1) + num2 * (multiplier - 1) + abs(y2 - y1) + num1 * (multiplier - 1)
        distances.append(distance)

    return sum(distances)

if __name__ == "__main__":
    input = list(sys.stdin.read().strip().splitlines())
    # input = list(example.strip().splitlines())

    result_part1 = solve(input, 2)
    print(f"Part 1: {result_part1}")

    result_part2 = solve(input, 1000000)
    print(f"Part 2: {result_part2}")
