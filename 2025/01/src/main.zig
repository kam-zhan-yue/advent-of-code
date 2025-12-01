const std = @import("std");
const _01 = @import("_01");

// const ArrayList = std.ArrayList;
// const test_allocator = std.testing.allocator;

const print = std.debug.print;
var gpa =  std.heap.GeneralPurposeAllocator(.{}){};
const allocator = gpa.allocator();

pub fn main() !void {
    // Init data structures
    var dial: i32 = 50;
    var turns: u32 = 0;

    // File Operations
    const file = try std.fs.cwd().openFile("input.txt", .{});
    defer file.close();
    var file_buffer: [4096]u8 = undefined;
    var reader = file.reader(&file_buffer);
    while (try reader.interface.takeDelimiter('\n')) |line| {
        const direction: u8 = line[0];
        const length: usize = line.len;
        const valueString: []const u8 = line[1..length];
        const value = try std.fmt.parseInt(i32, valueString, 10);

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
    }
    print("Turns to 0: {d}\n", .{turns});
}

test "test shitty filesystem library" {
    const file = try std.fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var file_buffer: [4096]u8 = undefined;
    var reader = file.reader(&file_buffer);
    var line_no: usize = 0;
    while (try reader.interface.takeDelimiter('\n')) |line| {
        line_no += 1;
        print("{d}--{s}\n", .{line_no, line});
    }
}
