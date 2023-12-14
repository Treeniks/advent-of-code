exception ParseError of string

let ( << ) f g x = f (g x)
let ( >> ) f g x = g (f x)

let count_char string char =
  String.to_seq string |> Seq.filter (( == ) char) |> Seq.length

let move_left row =
  String.split_on_char '#' row
  |> List.map (fun s -> (count_char s 'O', String.length s))
  |> List.map (fun (os, l) -> String.make os 'O' ^ String.make (l - os) '.')
  |> List.fold_left (fun acc s -> acc ^ "#" ^ s) ""

let load row =
  String.to_seq row
  |> Seq.mapi (fun i c -> if c = 'O' then String.length row - i else 0)
  |> Seq.fold_left ( + ) 0

let rec transpose : 'a list list -> 'a list list = function
  | [] -> []
  | lst :: [] -> List.map (fun l -> [ l ]) lst
  | lst :: lsts ->
      transpose lsts |> List.mapi (fun i l -> [ List.nth lst i ] @ l)

let parse_input input =
  String.trim input |> String.split_on_char '\n'
  |> List.map (String.to_seq >> List.of_seq)
  |> transpose
  |> List.map (List.to_seq >> String.of_seq)

let part1 input =
  parse_input input |> List.map (move_left >> load) |> List.fold_left ( + ) 0

let part2 input = 0

let example =
  {|O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
|}

let%test _ = part1 example = 136
let%test _ = part2 example = 0
