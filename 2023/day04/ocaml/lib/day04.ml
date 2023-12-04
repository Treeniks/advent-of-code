open List

let (<<) f g x = f (g x)

(* let sum = fold_left (+) 0 *)
(* let rec take n l = *)
(*   match (n, l) with *)
(*   | (_, []) -> [] *)
(*   | (0, _) -> [] *)
(*   | (n, x::xs) -> x :: take (n - 1) xs *)

let contains a = exists ((==) a)

let points len =
  let rec aux len acc =
    match len with
    | 0 -> acc
    | l -> aux (l - 1) (acc * 2)
  in
  if len == 0 then 0 else aux (len - 1) 1

let parse_numbers (l : string) =
  (String.split_on_char ' ' l) |> filter (fun s -> String.length s > 0) |> map (int_of_string << String.trim)

let parse_line l =
  let split = nth (String.split_on_char ':' l) 1 |> String.split_on_char '|' |> map String.trim in
  let winners = hd split in
  let owned = hd (tl split) in
  (parse_numbers winners, parse_numbers owned)

let part1_lines lines =
  let rec aux lines acc =
    match lines with
    | [] -> acc
    | l::ls ->
        let (winners, owned) = parse_line l in
        let l_points = filter (fun i -> contains i winners) owned |> length |> points in
        aux ls (acc + l_points)
  in
  aux lines 0

let part2_lines lines =
  let rec aux lines n acc =
    match (lines, n) with
    | ([], _) -> acc
    | (_, 0) -> acc
    | (l::ls, n) ->
        let (winners, owned) = parse_line l in
        let l_amount = filter (fun i -> contains i winners) owned |> length in
        aux ls (n - 1) (aux ls l_amount acc + l_amount)
  in
  let l_len = length lines in
  aux lines l_len l_len

let common f input =
  String.trim input |> String.split_on_char '\n' |> f

let part1 = common part1_lines
let part2 = common part2_lines

let example= {|Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
|}

let%test _ = part1 example = 13
let%test _ = part2 example = 30

