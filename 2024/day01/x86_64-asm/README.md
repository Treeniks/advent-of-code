# 2024 Day01 x86_64 Assembly

Requires a libc and assumes System V ABI for some IO-related function calls.

This implementation shits itself for ill-mannered inputs. Specifically: If the lines don't contain exactly 2 integers separated by space, I have no idea what happens, as I don't check for errors after the `strtoll` call. Also, I have a max on 0x1000 lines, because I didn't want to implement dynamic resizing of my array (wouldn't be too hard, but I'm good for now). And probably some other edge-cases that I didn't think about. All solvable, but this took too long as it is.

I do, however, trim each line and skip if it is empty (assuming my checks work that is...I didn't really test).

## Run With Input

The program assumes to find the input in a `input.txt` file.
```sh
just run
```
