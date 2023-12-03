const std = @import("std");
const testing = std.testing;

const COLORS = [_][]const u8{
    "blue",
    "green",
    "red",
};

const ColorMap = struct {
    const Self = @This();

    const ColorError = error{
        ColorError,
    };

    const map = std.ComptimeStringMap(usize, .{
        .{ "blue", 0 },
        .{ "green", 1 },
        .{ "red", 2 },
    });

    sums: [3]usize = .{0} ** 3,

    fn add(self: *Self, color: []const u8, value: usize) !void {
        if (map.get(color)) |c| {
            if (self.sums[c] < value) {
                self.sums[c] = value;
            }
        } else {
            return error.ColorError;
        }
    }

    fn get(self: Self, color: []const u8) !usize {
        if (map.get(color)) |c| {
            return self.sums[c];
        } else {
            return error.ColorError;
        }
    }
};

fn colorMapAndGameIDFromLine(line: []const u8) !?struct { map: ColorMap, id: usize } {
    if (std.mem.eql(u8, std.mem.trim(u8, line, &std.ascii.whitespace), "")) {
        // empty line
        return null;
    }

    var color_split = std.mem.splitScalar(u8, line, ':');

    const game = color_split.next().?;
    var it_game = std.mem.splitScalar(u8, game, ' ');
    _ = it_game.next();
    const game_id_s = it_game.next().?;
    const game_id = try std.fmt.parseInt(usize, game_id_s, 10);

    const draws = color_split.next().?;

    const map = try colorMapFromDraws(draws);

    return .{ .map = map, .id = game_id };
}

fn colorMapFromDraws(draws: []const u8) !ColorMap {
    var map = ColorMap{};

    var it_draws = std.mem.splitAny(u8, draws, ",;");
    while (it_draws.next()) |draw| {
        const d = std.mem.trim(u8, draw, &std.ascii.whitespace);

        var it = std.mem.splitScalar(u8, d, ' ');

        const val_s = it.next().?;
        const val = try std.fmt.parseInt(usize, val_s, 10);

        const color = it.next().?;

        try map.add(color, val);
    }

    return map;
}

fn common(input: []const u8, reduce: *const fn (map: ColorMap, id: usize, sum: *usize) ColorMap.ColorError!void) !usize {
    var sum: usize = 0;

    var it_lines = std.mem.splitScalar(u8, input, '\n');
    while (it_lines.next()) |line| {
        if (try colorMapAndGameIDFromLine(line)) |result| {
            const map = result.map;
            const game_id = result.id;

            try reduce(map, game_id, &sum);
        }
    }
    return sum;
}

fn part1_reduce(map: ColorMap, id: usize, sum: *usize) !void {
    if (try map.get("blue") <= 14 and try map.get("green") <= 13 and try map.get("red") <= 12) {
        sum.* += id;
    }
}

fn part1(input: []const u8) !usize {
    return common(input, part1_reduce);
}

fn part2_reduce(map: ColorMap, id: usize, sum: *usize) !void {
    _ = id;
    const power = try map.get("blue") * try map.get("green") * try map.get("red");
    sum.* += power;
}

fn part2(input: []const u8) !usize {
    return common(input, part2_reduce);
}

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    const stdin = std.io.getStdIn();

    const allocator = std.heap.page_allocator;

    const input = try stdin.readToEndAlloc(allocator, std.math.maxInt(usize));
    defer allocator.free(input);

    const result_part1 = try part1(input);
    const result_part2 = try part2(input);

    try stdout.print("Part 1: {d}\nPart 2: {d}\n", .{ result_part1, result_part2 });
}

test "part1 example" {
    const input =
        \\Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        \\Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        \\Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        \\Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        \\Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    ;

    const expected: usize = 8;
    const result = try part1(input);

    try testing.expectEqual(expected, result);
}

test "part2 example" {
    const input =
        \\Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        \\Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        \\Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        \\Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        \\Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    ;

    const expected: usize = 2286;
    const result = try part2(input);

    try testing.expectEqual(expected, result);
}
