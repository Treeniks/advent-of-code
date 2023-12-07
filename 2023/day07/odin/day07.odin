package main

import "core:fmt"
import "core:os"

main :: proc() {
    data, ok := os.read_entire_file("input.txt")
    if !ok {
        return
    }
    defer delete(data)

    input := string(data)
    p1, err := part1(input)
    if err != nil {
        return
    }

    fmt.println("Part 1:", p1)
}
