module Day03

export part1

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
    for (i, line) in enumerate(lines)
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

end # module Day03

using .Day03

function main()
    println("Part 1: ", part1(read(stdin, String)))
end

if abspath(PROGRAM_FILE) == @__FILE__
    main()
end

