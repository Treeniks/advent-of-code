import Lib
import Test.HUnit

testPart1 :: Test
testPart1 = TestCase (assertEqual "Part 1 Failed" 35 (part1 example))

testPart2 :: Test
testPart2 = TestCase (assertEqual "Part 2 Failed" 46 (part2 example))

main :: IO Counts
main = runTestTT $ TestList [testPart1, testPart2]
