# 2024 Day01 x86_64 Assembly

Requires a libc and assumes System V ABI for some IO-related function calls.

There is probably some edge-case I am not checking that may break everything, but it should check for most issues.

## Run With Input

The program takes the input as a filename CLI argument:
```sh
just build
./day01 input.txt
# or
just r input.txt
```
