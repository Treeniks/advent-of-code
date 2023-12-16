import strutils, sequtils

type grid[T] = seq[seq[T]]

func `[]`[T](grid: grid[T], index: tuple[x: int, y: int]): T =
  grid[index.y][index.x]

func `[]`[T](grid: var grid[T], index: tuple[x: int, y: int]): var T =
  grid[index.y][index.x]

func `[]=`[T](grid: var grid[T], index: tuple[x: int, y: int], value: T) =
  grid[index.y][index.x] = value

func `+`(a: tuple[x: int, y: int], b: tuple[x: int, y: int]): tuple[x: int, y: int] =
  (a.x + b.x, a.y + b.y)

proc `+=`(a: var tuple[x: int, y: int], b: tuple[x: int, y: int]) =
  a.x += b.x
  a.y += b.y

func inbound[T](index: tuple[x: int, y: int], grid: grid[T]): bool =
  index.y >= 0 and index.y < grid.len and index.x >= 0 and index.x < grid[0].len

proc castRay(grid: grid[char], start: tuple[x: int, y: int], direction: tuple[x: int, y: int], hits: var grid[seq[tuple[x: int, y: int]]]) =
  # cycle detection
  if not (start.inbound grid) or (direction in hits[start]): return

  var current = start
  hits[current].add(direction)
  while grid[current] == '.':
    current += direction
    if not current.inbound grid:
      return
    hits[current].add(direction)

  case grid[current]
  of '|':
    if direction.x == 0: castRay(grid, current + direction, direction, hits)
    else:
      castRay(grid, current + (0, 1), (0, 1), hits)
      castRay(grid, current + (0, -1), (0, -1), hits)
  of '-':
    if direction.y == 0: castRay(grid, current + direction, direction, hits)
    else:
      castRay(grid, current + (1, 0), (1, 0), hits)
      castRay(grid, current + (-1, 0), (-1, 0), hits)
  of '\\':
    if direction.x == 1: castRay(grid, current + (0, 1), (0, 1), hits)
    elif direction.x == -1: castRay(grid, current + (0, -1), (0, -1), hits)
    elif direction.y == 1: castRay(grid, current + (1, 0), (1, 0), hits)
    elif direction.y == -1: castRay(grid, current + (-1, 0), (-1, 0), hits)
  of '/':
    if direction.x == 1: castRay(grid, current + (0, -1), (0, -1), hits)
    elif direction.x == -1: castRay(grid, current + (0, 1), (0, 1), hits)
    elif direction.y == 1: castRay(grid, current + (-1, 0), (-1, 0), hits)
    elif direction.y == -1: castRay(grid, current + (1, 0), (1, 0), hits)
  else:
    discard

func parseInput(input: string): grid[char] =
  input.strip().splitLines().map(proc(s: string): seq[char] = s.items().toSeq())

proc part1*(input: string): int =
  let grid = parseInput(input)
  var hits: grid[seq[tuple[x: int, y: int]]] = @[]
  for line in grid:
    var tmp: seq[seq[tuple[x: int, y: int]]] = @[]
    for c in line:
      tmp.add(@[])
    hits.add(tmp)

  castRay(grid, (0, 0), (1, 0), hits)
  for line in hits:
    for b in line:
      if b.len > 0: result += 1

func part2*(input: string): int = 0

when isMainModule:
  let input = readAll(stdin)

  echo "Part 1: " & $part1(input)
  # echo "Part 2: " & $part2(input)
