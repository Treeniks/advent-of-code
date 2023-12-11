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

def insert_column(input, column, insert):
    for i in range(0, len(input)):
        input[i] = input[i][:column] + insert[i] + input[i][column:]

def expand(input):
    insert_r = "." * len(input[0])
    to_insert = []
    for row in range(0, len(input)):
        if no_galaxies(input[row]):
            to_insert.append(row)

    counter = 0
    for row in to_insert:
        input.insert(row + counter, insert_r)
        counter += 1

    insert_c = "." * len(input)
    to_insert = []
    for column in range(0, len(input[0])):
        if no_galaxies(get_column(input, column)):
            to_insert.append(column)

    counter = 0
    for column in to_insert:
        insert_column(input, column + counter, insert_c)
        counter += 1

def get_galaxies(input):
    result = []
    for row in range(0, len(input)):
        for column in range(0, len(input[0])):
            if input[row][column] == '#':
                result.append((row, column))
    return result

def part1(input):
    expand(input)
    galaxies = get_galaxies(input)

    distances = []

    for ((x1, y1), (x2, y2)) in itertools.combinations(galaxies, 2):
        distance = abs(x2 - x1) + abs(y2 - y1)
        distances.append(distance)

    return sum(distances)

if __name__ == "__main__":
    input = list(sys.stdin.read().strip().splitlines())

    result = part1(input)
    print(f"Part 1: {result}")
