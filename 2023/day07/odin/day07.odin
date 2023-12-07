package main

import "core:fmt"
import "core:os"

HAND_LENGTH :: 5

HandType :: enum {
    Five_Of_A_Kind,
    Four_Of_A_Kind,
    Full_House,
    Three_Of_A_Kind,
    Two_Pair,
    One_Pair,
    High_Card,
}

Hand :: struct {
    type: HandType,
    worth: int,
    s: string,
}

evaluate_handtype :: proc(input: string, get_same_higher_lower: proc(string) -> (int, int)) -> HandType {
    assert(len(input) == HAND_LENGTH)

    // check how many of the same card is in the hand
    same_higher, same_lower := get_same_higher_lower(input)

    switch same_higher {
        case 5: return .Five_Of_A_Kind
        case 4: return .Four_Of_A_Kind
        case 3:
            switch same_lower {
                case 2: return .Full_House
                case: return .Three_Of_A_Kind
            }
        case 2:
            switch same_lower {
                case 2: return .Two_Pair
                case: return .One_Pair
            }
        case: return .High_Card
    }
}

main :: proc() {
    data, ok := os.read_entire_file("input.txt")
    if !ok {
        return
    }
    defer delete(data)

    input := string(data)

    p1, err1 := part1(input)
    if err1 != nil {
        return
    }

    fmt.println("Part 1:", p1)

    p2, err2 := part2(input)
    if err2 != nil {
        return
    }
    fmt.println("Part 2:", p2)
}
