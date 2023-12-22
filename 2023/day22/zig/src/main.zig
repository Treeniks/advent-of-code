const std = @import("std");
const testing = std.testing;
const Allocator = std.mem.Allocator;

const example =
    \\1,0,1~1,2,1
    \\0,0,2~2,0,2
    \\0,2,3~2,2,3
    \\0,0,4~0,2,4
    \\2,0,5~2,2,5
    \\0,1,6~2,1,6
    \\1,1,8~1,1,9
;

// examples taken from reddit
const example2 =
    \\0,0,1~0,1,1
    \\1,1,1~1,1,1
    \\0,0,2~0,0,2
    \\0,1,2~1,1,2
;

const example3 =
    \\0,0,1~1,0,1
    \\0,1,1~0,1,2
    \\0,0,5~0,0,5
    \\0,0,4~0,1,4
;

// modified example from my input
const example4 =
    \\5,0,1~8,0,1
    \\1,4,2~1,4,3
    \\2,1,4~2,3,4
    \\2,3,5~2,4,5
    \\7,5,6~7,7,6
    \\6,9,7~6,9,7
    \\9,5,8~9,6,8
    \\7,0,9~9,0,9
;

const Pos3 = struct {
    x: usize,
    y: usize,
    z: usize,
};

const Brick = struct {
    const Self = @This();

    begin: Pos3,
    end: Pos3,

    // both minVals and maxVals is probably not even needed
    // since it would be much nicer to instead enforce an ordering
    // when creating bricks...oh well
    fn minVals(self: Self) Pos3 {
        const min_x = if (self.begin.x < self.end.x) self.begin.x else self.end.x;
        const min_y = if (self.begin.y < self.end.y) self.begin.y else self.end.y;
        const min_z = if (self.begin.z < self.end.z) self.begin.z else self.end.z;

        return .{
            .x = min_x,
            .y = min_y,
            .z = min_z,
        };
    }

    fn maxVals(self: Self) Pos3 {
        const max_x = if (self.begin.x > self.end.x) self.begin.x else self.end.x;
        const max_y = if (self.begin.y > self.end.y) self.begin.y else self.end.y;
        const max_z = if (self.begin.z > self.end.z) self.begin.z else self.end.z;

        return .{
            .x = max_x,
            .y = max_y,
            .z = max_z,
        };
    }
};

const ParseError = error.ParseError;

fn parseLine(line: []const u8) !Brick {
    var it = std.mem.splitScalar(u8, line, '~');
    const first = it.next() orelse return ParseError;
    const second = it.next() orelse return ParseError;

    var it_first = std.mem.splitScalar(u8, first, ',');
    const x1 = try std.fmt.parseInt(usize, it_first.next() orelse return ParseError, 10);
    const y1 = try std.fmt.parseInt(usize, it_first.next() orelse return ParseError, 10);
    const z1 = try std.fmt.parseInt(usize, it_first.next() orelse return ParseError, 10);

    var it_second = std.mem.splitScalar(u8, second, ',');
    const x2 = try std.fmt.parseInt(usize, it_second.next() orelse return ParseError, 10);
    const y2 = try std.fmt.parseInt(usize, it_second.next() orelse return ParseError, 10);
    const z2 = try std.fmt.parseInt(usize, it_second.next() orelse return ParseError, 10);

    return .{
        .begin = .{ .x = x1, .y = y1, .z = z1 },
        .end = .{ .x = x2, .y = y2, .z = z2 },
    };
}

fn parseInput(allocator: Allocator, input: []const u8) ![]Brick {
    const trimmed = std.mem.trim(u8, input, &std.ascii.whitespace);

    var list = std.ArrayList(Brick).init(allocator);

    var it_lines = std.mem.splitScalar(u8, trimmed, '\n');

    while (it_lines.next()) |line| {
        try list.append(try parseLine(line));
    }

    return list.toOwnedSlice();
}

fn cmpBrickZ(context: void, left: Brick, right: Brick) bool {
    _ = context;
    const min_left = left.minVals();
    const min_right = right.minVals();

    if (min_left.z < min_right.z) {
        return true;
    } else if (min_left.z > min_right.z) {
        return false;
    } else if (min_left.y < min_right.y) {
        return true;
    } else if (min_left.y > min_right.y) {
        return false;
    } else if (min_left.x < min_right.x) {
        return true;
    } else { // min_left.x > min_right.x or the two bricks are the same
        return false;
    }
}

// returns true if the two boxed overlop
fn checkOverlap(b1: Brick, b2: Brick) bool {
    const min_b1 = b1.minVals();
    const max_b1 = b1.maxVals();

    const min_b2 = b2.minVals();
    const max_b2 = b2.maxVals();

    const x_overlap = !(max_b1.x < min_b2.x or max_b2.x < min_b1.x);
    const y_overlap = !(max_b1.y < min_b2.y or max_b2.y < min_b1.y);
    const z_overlap = !(max_b1.z < min_b2.z or max_b2.z < min_b1.z);

    return x_overlap and y_overlap and z_overlap;
}

// returns true if b1 supports b2
fn checkSupport(b1: Brick, b2: Brick) bool {
    const min_b1 = b1.minVals();
    const max_b1 = b1.maxVals();

    const min_b2 = b2.minVals();
    const max_b2 = b2.maxVals();

    const x_overlap = !(max_b1.x < min_b2.x or max_b2.x < min_b1.x);
    const y_overlap = !(max_b1.y < min_b2.y or max_b2.y < min_b1.y);

    return max_b1.z == min_b2.z - 1 and (x_overlap and y_overlap);
}

fn makeBricksFall(bricks: []Brick) void {
    // sort by z
    std.mem.sort(Brick, bricks, {}, cmpBrickZ);

    for (bricks, 0..) |*brick, i| {
        outer: while (true) {
            if (brick.begin.z == 1 or brick.end.z == 1) break;

            // move down by one z
            brick.begin.z -= 1;
            brick.end.z -= 1;

            // check if we now have a collision with any brick below
            for (0..i) |j| {
                const other = bricks[j];
                if (checkOverlap(brick.*, other)) {
                    brick.begin.z += 1;
                    brick.end.z += 1;

                    break :outer;
                }
            }
        }
    }
}

fn countSupporters(bricks: []const Brick, i: usize) usize {
    var result: usize = 0;

    for (0..i) |j| {
        if (checkSupport(bricks[j], bricks[i])) {
            result += 1;
        }
    }

    return result;
}

fn countNonSupporting(bricks: []const Brick) !usize {
    var result: usize = 0;

    outer: for (bricks, 0..) |brick, i| {
        for (i + 1..bricks.len) |j| {
            if (checkSupport(brick, bricks[j])) {
                const supporters = countSupporters(bricks, j);

                if (supporters == 1) {
                    continue :outer;
                }
            }
        }

        result += 1;
    }

    return result;
}

fn countChain(bricks: *std.ArrayList(Brick), brick: Brick, i: usize) usize {
    var result: usize = 0;

    var j: usize = i;
    while (j < bricks.items.len) {
        if (checkSupport(brick, bricks.items[j])) {
            const supporters = countSupporters(bricks.items, j);

            // we check for 0
            // because the actual supporter was already removed by the caller
            if (supporters == 0) {
                result += 1;
                const new_brick = bricks.orderedRemove(j);
                result += countChain(bricks, new_brick, j);
                continue;
            }
        }

        j += 1;
    }

    return result;
}

fn part1(allocator: Allocator, input: []const u8) !usize {
    const bricks = try parseInput(allocator, input);
    defer allocator.free(bricks);

    makeBricksFall(bricks);

    return countNonSupporting(bricks);
}

fn part2(allocator: Allocator, input: []const u8) !usize {
    const bricks = try parseInput(allocator, input);

    makeBricksFall(bricks);

    // in parseInput, we convert from an ArrayList to a slice
    // now we convert back...kinda stupid but whatever
    const bricks_list = std.ArrayList(Brick).fromOwnedSlice(allocator, bricks);
    defer bricks_list.deinit();

    var result: usize = 0;

    for (0..bricks_list.items.len) |i| {
        var bricks_clone = try bricks_list.clone();
        defer bricks_clone.deinit();

        const brick = bricks_clone.orderedRemove(i);

        const chain = countChain(&bricks_clone, brick, i);
        result += chain;
    }

    return result;
}

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    const stdin = std.io.getStdIn();

    const allocator = std.heap.page_allocator;

    const input = try stdin.readToEndAlloc(allocator, std.math.maxInt(usize));
    defer allocator.free(input);

    const result_part1 = try part1(allocator, input);
    try stdout.print("Part 1: {d}\n", .{result_part1});

    const result_part2 = try part2(allocator, input);
    try stdout.print("Part 2: {d}\n", .{result_part2});
}

test "part 1 example" {
    const allocator = std.testing.allocator;

    const expected: usize = 5;
    const result = try part1(allocator, example);

    try testing.expectEqual(expected, result);
}

test "part 1 example2" {
    const allocator = std.testing.allocator;

    const expected: usize = 3;
    const result = try part1(allocator, example2);

    try testing.expectEqual(expected, result);
}

test "part 1 example3" {
    const allocator = std.testing.allocator;

    const expected: usize = 2;
    const result = try part1(allocator, example3);

    try testing.expectEqual(expected, result);
}

test "part 1 example4" {
    const allocator = std.testing.allocator;

    const expected: usize = 6;
    const result = try part1(allocator, example4);

    try testing.expectEqual(expected, result);
}

test "part 2" {
    const allocator = std.testing.allocator;

    const expected: usize = 7;
    const result = try part2(allocator, example);

    try testing.expectEqual(expected, result);
}
