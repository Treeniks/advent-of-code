# 2023 Day03 Julia

## Run With Input

### Within Julia Shell

```julia
julia> using Day03
julia> part1(read("input.txt", String))
```

### Via Script

```sh
$ cat input.txt | julia src/Day03.jl
```

## Run Tests

```sh
$ julia --project=.
julia> ]
(Day03) pkg> test Day03
```
