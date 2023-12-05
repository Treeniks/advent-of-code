using Day03
using Test

const example_part1 = raw"""467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"""

@test part1(example_part1) == 4361

