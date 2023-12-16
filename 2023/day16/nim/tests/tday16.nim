import unittest, day16

const input = """.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"""

test "Part 1":
  check part1(input) == 46

test "Part 2":
  check part2(input) == 51
