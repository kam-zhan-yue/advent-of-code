const std = @import("std");

const print = std.debug.print;
var gpa = std.heap.GeneralPurposeAllocator(.{}){};
const allocator = gpa.allocator();

pub fn main() !void {
    // Init data structures
    var dial: i32 = 50;
    var turns: i32 = 0;
    var clicks: i32 = 0;

    // File Operations
    const file = try std.fs.cwd().openFile("../inputs/01.txt", .{});
    defer file.close();
    var file_buffer: [4096]u8 = undefined;
    var reader = file.reader(&file_buffer);
    while (try reader.interface.takeDelimiter('\n')) |line| {
        const direction: u8 = line[0];
        const length: usize = line.len;
        const valueString: []const u8 = line[1..length];
        const value = try std.fmt.parseInt(i32, valueString, 10);

        const started_at_zero = dial == 0;

        // Rotate the dial
        if (direction == 'L') {
            dial -= value;
        } else {
            dial += value;
        }
        // Mark if the dial is positive or negative
        const negative: bool = dial < 0;
        if (dial < 0) {
            dial *= -1;
        }

        // ======= CLICKS ======= //
        // If negative, then it has clicked once
        if ((!started_at_zero and negative) or dial == 0) {
            clicks += 1;
        }
        const over = @divFloor(dial, 100);
        if (over > 0) {
            clicks += over;
        }

        // ======= TURNS ======= //
        // Get the remainder
        const remainder = @mod(dial, 100);
        if (remainder == 0) {
            turns += 1;
        }

        // Now update the dial accordingly
        if (negative) {
            dial = 100 - remainder;
        } else {
            dial = remainder;
        }
        // Reset the dial
        if (dial == 100) {
            dial = 0;
        }
    }

    print("Part 1 | Turns to 0: {d}\n", .{turns});
    print("Part 2 | Clicks to 0: {d}\n", .{clicks});
}
