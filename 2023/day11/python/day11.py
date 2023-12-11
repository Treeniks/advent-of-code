import itertools
import sys

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

def empty_lines(input):
    result = []
    for i in range(0, len(input)):
        if all(c == '.' for c in input[i]):
            result.append(i)
    return result

def solve(input, multiplier):
    rows = list(input.strip().splitlines())
    columns = []
    for i in range(0, len(rows[0])):
        column = get_column(rows, i);
        columns.append(column)
    galaxies = get_galaxies(rows)

    empty_lines_rows = empty_lines(rows)
    empty_lines_columns = empty_lines(columns)

    distances = []
    for ((x1, y1), (x2, y2)) in itertools.combinations(galaxies, 2):
        # invariant: x1 < x2 and y1 < y2
        if x1 > x2:
            tmp = x1
            x1 = x2
            x2 = tmp
        if y1 > y2:
            tmp = y1
            y1 = y2
            y2 = tmp

        num1 = sum(1 if y1 < x and x < y2 else 0 for x in empty_lines_rows)
        num2 = sum(1 if x1 < y and y < x2 else 0 for y in empty_lines_columns)
        distance = abs(x2 - x1) + abs(y2 - y1) + num1 * (multiplier - 1) + num2 * (multiplier - 1)
        distances.append(distance)
    return sum(distances)

def part1(input):
    return solve(input, 2)

def part2(input):
    return solve(input, 1000000)

if __name__ == "__main__":
    input = sys.stdin.read()

    result_part1 = part1(input)
    print(f"Part 1: {result_part1}")

    result_part2 = part2(input)
    print(f"Part 2: {result_part2}")
