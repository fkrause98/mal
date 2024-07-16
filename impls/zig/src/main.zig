const std = @import("std");
const io = std.io;
const expect = std.testing.expect;
pub fn main() !void {
    // Prints to stderr (it's a shortcut based on `std.io.getStdErr()`)
    // std.debug.print("All your {s} are belong to us.\n", .{"codebase"});

    // // stdout is for the actual output of your application, for example if you
    // // are implementing gzip, then only the compressed bytes should be sent to
    // // stdout, not any debugging messages.
    // const stdout_file = std.io.getStdOut().writer();
    // var bw = std.io.bufferedWriter(stdout_file);
    // const stdout = bw.writer();

    // try stdout.print("Run `zig build test` to run the tests.\n", .{});

    // try bw.flush(); // don't forget to flush!
    while (true) {
        std.debug.print("user> ", .{});
        const stdin = io.getStdIn().reader();
        const input = try stdin.readUntilDelimiterAlloc(
            std.heap.page_allocator,
            '\n',
            8192,
        );
        defer std.heap.page_allocator.free(input);
        try rep(input);
        // std.debug.print('\n', .{});
    }
}

pub fn READ(input: []const u8) []const u8 {
    return input;
}

pub fn EVAL(input: []const u8) []const u8 {
    return input;
}

pub fn PRINT(input: []const u8) []const u8 {
    return input;
}

pub fn rep(input: []const u8) !void {
    try io.getStdOut().writer().print("{s}\n", .{input});
}
