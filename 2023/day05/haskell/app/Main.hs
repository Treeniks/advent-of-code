module Main (main) where

import Lib

main :: IO ()
main = do
  input <- getContents

  putStr "Part 1: "
  print $ part1 input

  putStr "Part 2: "
  print $ part2 input
