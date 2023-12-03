module Day03

export part1
export part2

function parse_number(input, i::Integer)
    bi = 0
    for k in i:-1:1
        if !isdigit(input[k])
            break
        end
        bi = k
    end

    ei = 0
    for k in i:length(input)
        if !isdigit(input[k])
            break
        end
        ei = k
    end

    return (bi, parse(Int, input[bi:ei]))
end

function get_numbers_surrounding(input, i::Integer, j::Integer)::Set
    numbers = Set()

    # go through all directions
    # will include x = y = 0
    # but by assumption, that is not a digit, so it's fine
    for x in -1:1
        for y in -1:1
            try
                c = input[i+y][j+x]
                if isdigit(c)
                    (bi, n) = parse_number(input[i+y], j + x)
                    push!(numbers, (i + y, bi, n))
                end
            catch e
                if !(e isa BoundsError)
                    rethrow()
                end
            end
        end
    end

    return numbers
end

function part1(input::AbstractString)::Integer
    numbers = Set()
    lines = split(input, '\n', keepempty=false)
    for (i, line) in enumerate(map(strip, lines))
        for (j, c) in enumerate(line)
            if !(isdigit(c) || c == '.')
                union!(numbers, get_numbers_surrounding(lines, i, j))
            end
        end
    end

    result = 0
    for (_, _, n) in numbers
        result += n
    end
    result
end

function part2(input::AbstractString)::Integer
    sum = 0

    lines = split(input, '\n', keepempty=false)
    for (i, line) in enumerate(map(strip, lines))
        for (j, c) in enumerate(line)
            if c == '*'
                numbers = get_numbers_surrounding(lines, i, j)
                if length(numbers) == 2
                    prod = 1
                    for (i, j, n) in numbers
                        prod *= n
                    end
                    sum += prod
                end
            end
        end
    end

    sum
end

const example = raw"""467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"""

function main()
    input = read(stdin, String)

    println("Part 1: ", part1(input))
    println("Part 2: ", part2(input))
end

if abspath(PROGRAM_FILE) == @__FILE__
    main()
end

end # module Day03
