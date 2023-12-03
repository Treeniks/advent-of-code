using Day03
using Test

const example = raw"""467..114..
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

@test part1(example) == 4361
@test part2(example) == 467835
