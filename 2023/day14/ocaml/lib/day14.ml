let ( >> ) f g x = g (f x)

let count_char string char =
  String.to_seq string |> Seq.filter (( == ) char) |> Seq.length

let move_left =
  let aux row =
    String.split_on_char '#' row
    |> List.map (fun s -> (count_char s 'O', String.length s))
    |> List.map (fun (os, l) -> String.make os 'O' ^ String.make (l - os) '.')
    |> fun l ->
    List.hd l ^ (List.tl l |> List.fold_left (fun acc s -> acc ^ "#" ^ s) "")
  in
  List.map aux

let load platform =
  let aux row =
    String.to_seq row
    |> Seq.mapi (fun i c -> if c = 'O' then String.length row - i else 0)
    |> Seq.fold_left ( + ) 0
  in
  List.map aux platform |> List.fold_left ( + ) 0

let rec transpose : 'a list list -> 'a list list = function
  | [] -> []
  | lst :: [] -> List.map (fun l -> [ l ]) lst
  | lst :: lsts ->
      transpose lsts |> List.mapi (fun i l -> [ List.nth lst i ] @ l)

(* rotate the platform counter-clockwise *)
let rotate_counter_clockwise platform =
  List.map (String.to_seq >> List.of_seq) platform
  |> transpose
  |> List.map (List.to_seq >> String.of_seq)
  |> List.rev

let rotate_clockwise platform =
  rotate_counter_clockwise platform
  |> List.map (fun s ->
         String.to_seq s |> List.of_seq |> List.rev |> List.to_seq
         |> String.of_seq)
  |> List.rev

let parse_input input = String.trim input |> String.split_on_char '\n'

let part1 input =
  parse_input input |> rotate_counter_clockwise |> move_left |> load

(* assumes north is left, then does one cycle, ending with north left again *)
let cycle_1 platform =
  move_left platform |> rotate_clockwise |> move_left |> rotate_clockwise
  |> move_left |> rotate_clockwise |> move_left |> rotate_clockwise

let cycle_n total platform =
  let rec aux n p cache =
    match n with
    | 0 -> p
    | n -> (
        match List.find_index (( = ) p) cache with
        | Some i ->
            let rest = n mod (i + 1) in
            List.nth cache (i - rest)
        | None -> aux (n - 1) (cycle_1 p) (p :: cache))
  in
  aux total platform []

let part2 input =
  parse_input input |> rotate_counter_clockwise |> cycle_n 1000000000 |> load

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
let%test _ = part2 example = 64
