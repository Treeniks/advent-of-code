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

let parse_input input parse_line =
  match String.trim input |> String.split_on_char '\n' with
  | [ linet; lined ] ->
      let t = parse_line linet in
      let d = parse_line lined in
      (t, d)
  | _ -> raise (ParseError "too many or too few lines")

let solver time distance =
  let t = float_of_int time in
  let d = float_of_int distance in

  (* both boundaries are exclusive *)
  let boundary_low =
    (t -. sqrt ((t *. t) -. (4. *. d))) /. 2. |> floor |> int_of_float
  in
  let boundary_high =
    (t +. sqrt ((t *. t) -. (4. *. d))) /. 2. |> ceil |> int_of_float
  in

  boundary_high - boundary_low - 1

let part1 input =
  let times, distances = parse_input input parse_line_part1 in
  map2 solver times distances |> fold_left ( * ) 1

let part2 input =
  let time, distance = parse_input input parse_line_part2 in
  solver time distance

let example = {|Time:      7  15   30
Distance:  9  40  200
|}

let%test _ = part1 example = 288
let%test _ = part2 example = 71503
