import day01.{part1, part2}
import gleeunit
import gleeunit/should

const input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"

pub fn main() {
  gleeunit.main()
}

pub fn part1_test() {
  let actual = part1(input)
  let expected = 24_000
  actual |> should.equal(expected)
}

pub fn part2_test() {
  let actual = part2(input)
  let expected = 45_000
  actual |> should.equal(expected)
}
