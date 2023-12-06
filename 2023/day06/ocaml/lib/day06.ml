open List

exception ParseError of string

let ( << ) f g x = f (g x)

let parse_line_part1 (line : string) : int list =
  match String.split_on_char ':' line with
  | [ _; l ] ->
      String.trim l |> String.split_on_char ' '
      |> filter (( < ) 0 << String.length)
      |> map int_of_string
  | _ -> raise (ParseError "line is incorrect")

let parse_line_part2 (line : string) : int =
  match String.split_on_char ':' line with
  | [ _; l ] ->
      String.trim l |> String.split_on_char ' '
      |> filter (( < ) 0 << String.length)
      |> fold_left ( ^ ) "" |> int_of_string
  | _ -> raise (ParseError "line is incorrect")

let len_achieved time_total time_pressed =
  let time_moving = time_total - time_pressed in
  let speed = time_pressed in
  time_moving * speed

let dists time_total =
  (* tail recursive *)
  (* technically gives the distances in the wrong order *)
  (* but there is no need to reverse acc in the end as the order does not matter *)
  (* and if it did, we might as well just count up instead of down *)
  let rec aux time acc =
    if time == 0 then acc
    else aux (time - 1) (len_achieved time_total time :: acc)
  in
  (* assuming time_total != 0 *)
  aux (time_total - 1) []

(* original dists that resulted in a stackoverflow for part 2 *)
(* let dists time_total = *)
(*   let rec aux = function *)
(*     | 0 -> [] *)
(*     | time -> len_achieved time_total time :: aux (time - 1) *)
(*   in *)
(*   aux (time_total - 1) *)

let parse_input input parse_line =
  match String.trim input |> String.split_on_char '\n' with
  | [ linet; lined ] ->
      let t = parse_line linet in
      let d = parse_line lined in
      (t, d)
  | _ -> raise (ParseError "too many or too little lines")

let part1 input =
  let times, distances = parse_input input parse_line_part1 in
  let rec aux times distances acc =
    match (times, distances) with
    | [], [] -> acc
    | t :: ts, d :: ds ->
        aux ts ds ((dists t |> filter (( < ) d) |> length) * acc)
    | _ -> raise (ParseError "amount of times and distances is not the same")
  in
  aux times distances 1

let part2 input =
  let time, distance = parse_input input parse_line_part2 in
  dists time |> filter (( < ) distance) |> length

let example = {|Time:      7  15   30
Distance:  9  40  200
|}

let%test _ = part1 example = 288
let%test _ = part2 example = 71503
