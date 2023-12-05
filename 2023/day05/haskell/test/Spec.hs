import Test.HUnit
import Lib

testPart1 :: Test
testPart1 = TestCase (assertEqual "Part 1 Failed" 3 (part1 example_part1))

main :: IO Counts
main = runTestTT $ TestList [testPart1]
