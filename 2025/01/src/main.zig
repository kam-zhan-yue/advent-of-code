const std = @import("std");
const _01 = @import("_01");

const print = std.debug.print;
var gpa =  std.heap.GeneralPurposeAllocator(.{}){};
const allocator = gpa.allocator();

pub fn main() !void {
    var dial: i32 = 50;
    var turns: u32 = 0;
    const inputs = [_][]const u8{
        "L68",
        "L30",
        "R48",
        "L5",
        "R60",
        "L55",
        "L1",
        "L99",
        "R14",
        "L82",
    };
    for (inputs) |input| {
        const direction: u8 = input[0];
        const length: usize = input.len;
        const valueString: []const u8 = input[1..length];
        const value = try std.fmt.parseInt(i32, valueString, 10);

        print("{c} {d}\n", .{direction, value});
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
        print("Dial is now {d}\n", .{dial});
    }
    print("Turns to 0: {d}\n", .{turns});
}

