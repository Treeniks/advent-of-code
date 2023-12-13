require "spec"
require "../src/day13"

example = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"

it "test part1" do
  part1(example).should eq(405)
end

it "test part2" do
  part2(example).should eq(400)
end
