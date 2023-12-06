let read_until_eof () =
  let rec aux acc =
    try
      let line = read_line () in
      aux (acc ^ line ^ "\n")
    with End_of_file -> acc
  in
  aux ""

let () =
  let input = read_until_eof () in

  print_string "Part 1: ";
  input |> Day06.part1 |> print_int;
  print_newline ();

  print_string "Part 2: ";
  input |> Day06.part2 |> print_int;
  print_newline ();

  ()
