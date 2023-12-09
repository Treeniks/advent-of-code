import unittest, day09

const input = """0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"""

test "Part 1":
  check part1(input) == 114
