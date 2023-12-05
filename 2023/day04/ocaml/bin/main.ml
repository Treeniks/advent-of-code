let read_until_eof () =
  let rec aux acc =
    try
      let line = read_line () in
      aux (acc ^ line ^ "\n")
    with
    | End_of_file -> acc
  in
  aux ""

let () =
  print_string "Part 1: ";
  let input = read_until_eof () in
  input |> Day04.part1 |> print_int

