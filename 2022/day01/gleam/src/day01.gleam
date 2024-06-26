import gleam/erlang
import gleam/int
import gleam/io
import gleam/list
import gleam/string
import gleam/string_builder

fn read_stdin_loop(b: string_builder.StringBuilder) {
  case erlang.get_line("") {
    Ok(s) -> string_builder.append(b, s) |> read_stdin_loop()
    Error(_) -> b
  }
}

fn read_stdin() {
  let b = string_builder.new()
  read_stdin_loop(b) |> string_builder.to_string
}

fn parse_input(input: String) {
  input
  |> string.trim
  |> string.split(on: "\n")
  |> list.chunk(by: string.is_empty)
  |> list.filter(keeping: fn(l) {
    l
    |> list.filter(keeping: fn(s) { !string.is_empty(s) })
    |> fn(s) { !list.is_empty(s) }
  })
  |> list.map(with: fn(l) {
    l
    |> list.map(with: fn(line) {
      case int.parse(line) {
        Ok(i) -> i
        Error(_) -> panic
      }
    })
    |> int.sum
  })
}

pub fn part1(input: String) {
  parse_input(input)
  |> list.fold(from: 0, with: int.max)
}

pub fn part2(input: String) {
  let sort = fn(triple) {
    let #(a, b, c) = triple
    let #(a, b, c) = case a < b {
      True -> #(b, a, c)
      False -> #(a, b, c)
    }
    let #(a, b, c) = case a < c {
      True -> #(c, b, a)
      False -> #(a, b, c)
    }
    case b < c {
      True -> #(a, c, b)
      False -> #(a, b, c)
    }
  }

  parse_input(input)
  |> list.fold(from: #(0, 0, 0), with: fn(triple, x) {
    let #(a, b, c) = triple
    case x > c {
      True -> sort(#(a, b, x))
      False -> triple
    }
  })
  |> fn(triple) {
    let #(a, b, c) = triple
    a + b + c
  }
}

pub fn main() {
  let input = read_stdin()
  let p1 = part1(input)
  io.println("Part 1: " <> int.to_string(p1))

  let p2 = part2(input)
  io.println("Part 2: " <> int.to_string(p2))
}
