package main

import "core:strings"
import "core:runtime"
import "core:slice"
import "core:strconv"

@(private="file")
CardType :: enum {
    A, K, Q, J,
    T, N9, N8, N7, N6, N5, N4, N3, N2,
}

@(private="file")
card_of_u8 :: proc(c: u8) -> CardType {
    switch c {
        case 'A': return .A
        case 'K': return .K
        case 'Q': return .Q
        case 'J': return .J
        case 'T': return .T
        case '9': return .N9
        case '8': return .N8
        case '7': return .N7
        case '6': return .N6
        case '5': return .N5
        case '4': return .N4
        case '3': return .N3
        case: return .N2
    }
}

@(private="file")
hand_less :: proc(i: Hand, j: Hand) -> bool {
    if i.type == j.type {
        for k := 0; k < HAND_LENGTH; k += 1 {
            if i.s[k] == j.s[k] { continue }

            i_card := card_of_u8(i.s[k])
            j_card := card_of_u8(j.s[k])

            return i_card < j_card
        }
    }

    return i.type < j.type
}

@(private="file")
get_same_higher_lower :: proc(input: string) -> (same_higher: int, same_lower: int) {
    higher_symbol: u8
    for i := 0; i < len(input); i += 1 {
        tmp := 0
        symbol := input[i]

        if symbol == higher_symbol { continue }

        for j := i; j < len(input); j += 1 {
            if input[j] == symbol {
                tmp += 1
            }
        }

        if tmp >= same_higher {
            same_lower = same_higher
            same_higher = tmp
            higher_symbol = symbol
        } else if tmp > same_lower {
            same_lower = tmp
        }
    }

    return
}

part1 :: proc(input: string) -> (result: int, err: runtime.Allocator_Error) {
    trimmed := strings.trim_space(input)
    lines := strings.split_lines(trimmed) or_return

    hands := make([]Hand, len(lines))
    for line, i in lines {
        trimmed_line := strings.trim_space(line)
        tmp := strings.split(trimmed_line, " ") or_return
        hand := tmp[0]
        value := tmp[1]

        hand_type := evaluate_handtype(hand, get_same_higher_lower)
        hands[i] = Hand{hand_type, strconv.atoi(value), hand}
    }

    slice.sort_by(hands[:], hand_less)

    for hand, i in hands {
        result += (len(hands) - i) * hand.worth
    }

    return
}
