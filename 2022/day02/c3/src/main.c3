module day02;
import std::io;

fn int points_of_line_part1(char opponent, char you)
{
    int points = 0;

    points += you + 1;
    if (opponent == you) points += 3;
    if ((opponent + 1) % 3 == you) points += 6;

    return points;
}

fn int points_of_line_part2(char opponent, char outcome)
{
    int points = 1;
    switch (outcome)
    {
        case 0: points += ((opponent - 1) + 3) % 3;
        case 1: points += opponent + 3;
        case 2: points += ((opponent + 1) % 3) + 6;
    }
    return points;
}

def PointLine = fn int(char, char);
fn int solve(String input, PointLine points_of_line)
{
    String[] lines = input.trim().split("\n");
    int points = 0;
    foreach (line : lines)
    {
        char first = line[0] - 'A';
        char second = line[2] - 'X';

        points += points_of_line(first, second);
    }
    return points;
}

fn int part1(String input)
{
    return solve(input, &points_of_line_part1);
}

fn int part2(String input)
{
    return solve(input, &points_of_line_part2);
}

fn void! main(String[] args)
{
    String input = (String)io::file::load_new("input.txt")!;

    int p1 = part1(input);
    io::printn(p1);

    int p2 = part2(input);
    io::printn(p2);
}

String test_input = `A Y
B X
C Z
`;

fn void test_part1() @test
{
    assert(part1(test_input) == 15);
}

fn void test_part2() @test
{
    assert(part2(test_input) == 12);
}
