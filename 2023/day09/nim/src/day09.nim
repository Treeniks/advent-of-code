import std/strutils
import std/sequtils

func parseLine(line: string): seq[int] =
  for value in line.splitWhitespace():
    result.add(value.parseInt())

proc differentiate(line: seq[int]): seq[seq[int]] =
  result = @[line]

  var current_line = line
  while true:
    var new_entry: seq[int] = @[]
    for i in countup(low current_line, pred high current_line):
      new_entry.add(current_line[succ i] - current_line[i])

    result.add(new_entry)
    current_line = new_entry

    if new_entry.all(proc (x: int): bool = x == 0):
      break

proc extrapolate_right(diffs: var seq[seq[int]]) =
  diffs[high diffs].add(0)
  for i in countdown(pred high diffs, low diffs):
    let current = diffs[i]
    let next = diffs[succ i]

    let left = current[high current]
    let res = next[high next]
    # res = right - left => right = res + left
    let right = res + left
    diffs[i].add(right)

# extremely inefficient
proc extrapolate_left(diffs: var seq[seq[int]]) =
  diffs[high diffs].add(0)
  for i in countdown(pred high diffs, low diffs):
    let current = diffs[i]
    let next = diffs[succ i]

    let right = current[low current]
    let res = next[low next]
    # res = right - left => left = right - res
    let left = right - res
    diffs[i].insert(left, low current)

func part1*(input: string): int =
  result = 0
  for line in input.strip().splitLines():
    var s = parseLine(line)
    var diffs = differentiate(s)
    extrapolate_right(diffs)

    let tmp = diffs[low diffs]
    result += tmp[high tmp]

func part2*(input: string): int =
  result = 0
  for line in input.strip().splitLines():
    var s = parseLine(line)
    var diffs = differentiate(s)
    extrapolate_left(diffs)

    let tmp = diffs[low diffs]
    result += tmp[low tmp]

when isMainModule:
  let input = readAll(stdin)

  echo "Part 1: " & $part1(input)
  echo "Part 2: " & $part2(input)
