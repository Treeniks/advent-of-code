module Lib
  ( part1,
    part2,
    example,
  )
where

import Data.Char
import Data.List (dropWhileEnd)

-- from inclusive to exclusive
data Range = Int :.. Int
  deriving (Show)

infix 5 :..

-- takes a list like ["79", "14", "55", "13"]
-- returns [79, 14, 55, 13]
parseInts :: [String] -> [Int]
parseInts = map read

-- takes a string like "seeds: 79 14 55 13"
-- returns [79, 14, 55, 13]
parseSeedsPart1 :: String -> [Int]
parseSeedsPart1 input = parseInts $ tail $ words input

-- takes a string like "seeds: 79 14 55 13"
-- returns [79 :.. 93, 55 :.. 68]
parseSeedsPart2 :: String -> [Range]
parseSeedsPart2 input = aux $ parseInts $ tail $ words input
  where
    aux :: [Int] -> [Range]
    aux [] = []
    aux [_] = error "incorrect number of seeds"
    aux (x1 : x2 : xs) = (x1 :.. x1 + x2) : aux xs

-- takes a list of lines like ["50 98 2", "52 50 48"]
-- returns the resulting mapping as a function on integers
-- {98 |-> 50, 99 |-> 51, 50 |-> 52, ..., 97 |-> 100}
parseMapPart1 :: [String] -> (Int -> Int)
parseMapPart1 = aux id
  where
    aux :: (Int -> Int) -> [String] -> (Int -> Int)
    aux m [] = m
    aux m (s : ss) = aux newMap ss
      where
        newMap :: (Int -> Int)
        newMap x =
          case parseInts $ words s of
            [dest, source, len] ->
              if x >= source && (source + len > x)
                then (x - source) + dest
                else m x
            _ -> error "incorrect number of values in map"

-- other Range functions that ended up being unnecessary
--
-- union :: Range -> Range -> [Range]
-- union (l1 :.. u1) (l2 :.. u2) =
--   if l1 <= u2 || l2 <= u1 then [min l1 l2 :.. max u1 u2] else [l1 :.. u1, l2 :.. u2]
--
-- difference :: Range -> Range -> [Range]
-- difference (l1 :.. u1) (l2 :.. u2)
--   | not $ overlap (l1 :.. u1) (l2 :.. u2) = [l1 :.. u1]
--   | l2 <= l1 && u1 <= u2 = []
--   | l1 < l2 && u2 < u1 = [l1 :.. l2, u2 :.. u1]
--   | l2 < l1 = [u2 :.. u1]
--   | otherwise = [l1 :.. l2]

overlap :: Range -> Range -> Bool
overlap (l1 :.. u1) (l2 :.. u2) = (l1 < u2 && u1 > l2) || (l2 < u1 && u2 > l1)

intersection :: Range -> Range -> Maybe Range
intersection (l1 :.. u1) (l2 :.. u2) =
  if overlap (l1 :.. u1) (l2 :.. u2) then Just (max l1 l2 :.. min u1 u2) else Nothing

-- takes a list of lines like ["50 98 2", "52 50 48"]
-- returns the resulting mapping as a function on ranges
-- the result of applying a range to this function is a list of ranges
-- as the range might get split in the process
parseMapPart2 :: [String] -> (Range -> [Range])
parseMapPart2 = aux (\x -> [x])
  where
    aux :: (Range -> [Range]) -> [String] -> (Range -> [Range])
    aux m [] = m
    aux m (s : ss) = aux newMap ss
      where
        newMap :: Range -> [Range]
        newMap (lower :.. upper) =
          case parseInts $ words s of
            [dest, source, len] ->
              case intersection (lower :.. upper) (source :.. source + len) of
                Nothing -> m (lower :.. upper)
                Just (l :.. u)
                  | l == lower && u == upper -> [l + diff :.. u + diff]
                  | l == lower && u < upper -> [l + diff :.. u + diff] ++ m (u :.. upper)
                  | l > lower && u == upper -> m (lower :.. l) ++ [l + diff :.. u + diff]
                  | otherwise -> m (lower :.. l) ++ [l + diff :.. u + diff] ++ m (u :.. upper)
              where
                diff = dest - source
            _ -> error "incorrect number of values in map"

findMinInRanges :: [Range] -> Int
findMinInRanges [] = error "cannot find minimum in no ranges"
findMinInRanges [(l :.. _)] = l
findMinInRanges ((l :.. _) : rs) = min l $ findMinInRanges rs

splitOnEmptyLines :: [String] -> [[String]]
splitOnEmptyLines input = aux input []
  where
    aux :: [String] -> [String] -> [[String]]
    aux [] current = [current]
    aux (line : rest) current = if (line == "") then current : aux rest [] else aux rest (current ++ [line])

part1 :: String -> Int
part1 input = aux maps seeds
  where
    trimmed = dropWhileEnd isSpace $ dropWhile isSpace input
    splitted = splitOnEmptyLines $ lines trimmed
    seeds = parseSeedsPart1 $ head $ head splitted
    maps = map (\b -> parseMapPart1 $ tail b) $ tail splitted

    aux :: [(Int -> Int)] -> [Int] -> Int
    aux [] xs = minimum xs
    aux (m : ms) xs = aux ms $ map m xs

part2 :: String -> Int
part2 input = findMinInRanges $ aux maps seeds
  where
    trimmed = dropWhileEnd isSpace $ dropWhile isSpace input
    splitted = splitOnEmptyLines $ lines trimmed
    seeds = parseSeedsPart2 $ head $ head splitted
    maps = map (\b -> parseMapPart2 $ tail b) $ tail splitted

    aux :: [Range -> [Range]] -> [Range] -> [Range]
    aux [] xs = xs
    aux (m : ms) xs = aux ms $ concat $ map m xs

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
