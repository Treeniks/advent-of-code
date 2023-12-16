let contains x l = List.exists (( = ) x) l

let cast_ray grid start direction =
  let rec aux current direction hits =
    if List.exists (fun (p, d) -> p = current && contains direction d) hits then
      hits
    else
      let x, y = current in
      let dx, dy = direction in
      try
        let c = List.nth (List.nth grid y) x in
        let hits =
          (* if the current position is already in hits, then add the current direction to it *)
          if List.exists (fun (p, _) -> p = current) hits then
            List.map
              (fun (p, d) ->
                if p != current then (p, d) else (p, direction :: d))
              hits
          else (current, [ direction ]) :: hits
        in
        match c with
        | '.' -> aux (x + dx, y + dy) direction hits
        | '|' ->
            if dx = 0 then aux (x, y + dy) direction hits
            else
              let hits = aux (x, y - 1) (0, -1) hits in
              aux (x, y + 1) (0, 1) hits
        | '-' ->
            if dy = 0 then aux (x + dx, y) direction hits
            else
              let hits = aux (x - 1, y) (-1, 0) hits in
              aux (x + 1, y) (1, 0) hits
        | '\\' -> (
            match (dx, dy) with
            | 1, _ -> aux (x, y + 1) (0, 1) hits
            | -1, _ -> aux (x, y - 1) (0, -1) hits
            | _, 1 -> aux (x + 1, y) (1, 0) hits
            | _, -1 -> aux (x - 1, y) (-1, 0) hits
            | _ -> hits)
        | '/' -> (
            match (dx, dy) with
            | 1, _ -> aux (x, y - 1) (0, -1) hits
            | -1, _ -> aux (x, y + 1) (0, 1) hits
            | _, 1 -> aux (x - 1, y) (-1, 0) hits
            | _, -1 -> aux (x + 1, y) (1, 0) hits
            | _ -> hits)
        | _ -> hits
      with _ -> hits
  in
  List.length (aux start direction [])

let parse_input input =
  let trimmed = String.trim input in
  let split = String.split_on_char '\n' trimmed in
  List.map (fun x -> List.of_seq (String.to_seq x)) split

let part1 input =
  let grid = parse_input input in
  cast_ray grid (0, 0) (1, 0)

let rec max_of_list acc = function
  | [] -> acc
  | x :: xs -> max_of_list (max x acc) xs

let part2 input =
  let grid = parse_input input in
  let max_x = List.length grid - 1 in
  let max_y =
    match grid with
    | line :: _ -> List.length line - 1
    | [] -> raise (Failure "Not a grid")
  in
  let starts_x = List.init max_x (fun x -> x) in
  let starts_y = List.init max_y (fun y -> y) in
  let rx =
    List.map
      (fun x ->
        [ cast_ray grid (x, 0) (0, 1); cast_ray grid (x, max_y) (0, -1) ])
      starts_x
    |> List.concat |> max_of_list 0
  in
  let ry =
    List.map
      (fun y ->
        [ cast_ray grid (0, y) (1, 0); cast_ray grid (max_x, y) (-1, 0) ])
      starts_y
    |> List.concat |> max_of_list 0
  in
  max rx ry

let example =
  {|.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
|}

let%test _ = part1 example = 46
let%test _ = part2 example = 51
