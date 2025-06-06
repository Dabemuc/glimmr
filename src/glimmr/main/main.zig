const std = @import("std");
const core = @import("cli_core");
const builtin = @import("builtin");

var directory: std.fs.Dir = undefined;
var input: []u8 = undefined;

pub fn main() !void {
    if (builtin.mode == .Debug)
        std.debug.print("== Running Glimmr in debug mode! ==\n", .{});

    const allocator = std.heap.page_allocator;
    var cli_helper = core.CliHelper.init(allocator);

    cli_helper.registerOption(.{
        .long_name = "directory",
        .short_name = 'd',
        .description = "The directory to visualise. Defaults to current directory.",
        .callback = setDirectory,
    });

    const stdIn = cli_helper.readStdIn(std.io.getStdIn()) catch |err| {
        std.debug.print("Failed to read stdIn\n{}", .{err});
        std.process.exit(1);
    };
    input = stdIn;
    try cli_helper.parseArgs(std.process.args());
}

fn setDirectory(dir: []const u8) void {
    directory = std.fs.openDirAbsolute(dir, .{ .iterate = true }) catch {
        std.debug.print("Failed to open {s}\n", .{dir});
        std.process.exit(1);
    };
}
