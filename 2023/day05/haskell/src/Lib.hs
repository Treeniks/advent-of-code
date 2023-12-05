module Lib
  ( part1,
    part2,
    example,
  )
where

import Data.Char
import Data.List (dropWhileEnd)

-- takes a list like
-- [79, 14, 55, 13]
--
-- returns a list of the integers
parseInts :: [String] -> [Int]
parseInts [] = []
parseInts (s : ss) = read s : parseInts ss

-- takes a string like
-- seeds: 79 14 55 13
--
-- returns a list of the integers
parseSeedsPart1 :: String -> [Int]
parseSeedsPart1 input = parseInts $ tail $ words input

parseSeedsPart2 :: String -> [Int]
parseSeedsPart2 input = aux $ parseInts $ tail $ words input
  where
    aux :: [Int] -> [Int]
    aux [] = []
    aux [_] = error "incorrect number of seeds"
    aux (x1 : x2 : xs) = [x1 .. x1 + x2] ++ aux xs

-- takes a list of lines like
-- 50 98 2
-- 52 50 48
--
-- returns the resulting mapping
parseMap :: [String] -> (Int -> Int)
parseMap =
  aux (\x -> x)
  where
    aux :: (Int -> Int) -> [String] -> (Int -> Int)
    aux m [] = m
    aux m (s : ss) = aux newMap ss
      where
        newMap :: (Int -> Int)
        newMap x =
          case parseInts $ words s of
            [dest, source, len] ->
              if (x >= source) && (source + len > x)
                then (x - source) + dest
                else m x
            _ -> error "incorrect number of ints in map"

splitOnEmptyLines :: [String] -> [[String]]
splitOnEmptyLines input = aux input []
  where
    aux :: [String] -> [String] -> [[String]]
    aux [] current = [current]
    aux (line : rest) current = if (line == "") then current : aux rest [] else aux rest (current ++ [line])

common :: (String -> [Int]) -> String -> Int
common parseSeeds input =
  aux maps seeds
  where
    trimmed = dropWhileEnd isSpace $ dropWhile isSpace input
    splitted = splitOnEmptyLines $ lines trimmed
    seeds = parseSeeds $ head $ head splitted
    maps = map (\b -> parseMap $ tail b) $ tail splitted

    aux :: [(Int -> Int)] -> [Int] -> Int
    aux [] xs = minimum xs
    aux (m : ms) xs = aux ms $ map m xs

part1 :: String -> Int
part1 = common parseSeedsPart1

part2 :: String -> Int
part2 = common parseSeedsPart2

example :: String
example =
  unlines
    [ "seeds: 79 14 55 13",
      "",
      "seed-to-soil map:",
      "50 98 2",
      "52 50 48",
      "",
      "soil-to-fertilizer map:",
      "0 15 37",
      "37 52 2",
      "39 0 15",
      "",
      "fertilizer-to-water map:",
      "49 53 8",
      "0 11 42",
      "42 0 7",
      "57 7 4",
      "",
      "water-to-light map:",
      "88 18 7",
      "18 25 70",
      "",
      "light-to-temperature map:",
      "45 77 23",
      "81 45 19",
      "68 64 13",
      "",
      "temperature-to-humidity map:",
      "0 69 1",
      "1 0 69",
      "",
      "humidity-to-location map:",
      "60 56 37",
      "56 93 4"
    ]
