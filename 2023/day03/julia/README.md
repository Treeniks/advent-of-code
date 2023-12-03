in julia console:
```julia
julia> using Day03
julia> part1(read("input.txt", String))
```

or directly via the script:
```sh
$ cat input.txt | julia src/Day03.jl
```

to run tests:
```sh
$ julia --project=.
julia> ]
(Day03) pkg> test Day03
```
