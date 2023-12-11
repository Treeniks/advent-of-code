import day11
import unittest

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

class TestExamples(unittest.TestCase):
    def test_part1(self):
        result_part1 = day11.part1(example)
        self.assertEqual(result_part1, 374)

    def test_solve(self):
        result_10 = day11.solve(example, 10)
        self.assertEqual(result_10, 1030)

        result_100 = day11.solve(example, 100)
        self.assertEqual(result_100, 8410)

if __name__ == "__main__":
    unittest.main()
