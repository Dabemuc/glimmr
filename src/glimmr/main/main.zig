const std = @import("std");
const core = @import("cli_core");
const builtin = @import("builtin");

var excludes_comma_separated_string: []const u8 = undefined;
var input: ?[]const u8 = null;

pub fn main() !void {
    if (builtin.mode == .Debug)
        std.debug.print("== Running Glimmr in debug mode! ==\n", .{});

    const allocator = std.heap.page_allocator;
    var cli_helper = core.CliHelper.init(allocator);

    cli_helper.registerOption(.{
        .long_name = "exclude",
        .short_name = 'x',
        .description = "The files or directories to exclude as comma-separated string. (Example: '-x file1,folder1,./folder2/file2')",
        .callback = setExcludes,
        .expects_parameter = true,
    });

    const stdIn = cli_helper.readStdIn(std.io.getStdIn(), allocator) catch |err| {
        std.debug.print("Failed to read stdIn\n{}", .{err});
        std.process.exit(1);
    };
    if (stdIn != null) {
        input = stdIn;
    }
    const inputAsArg = cli_helper.parseArgs(std.process.args()) catch |err| {
        std.debug.print("Failed to parse args\n{}", .{err});
        std.process.exit(1);
    };
    if (inputAsArg != null) {
        if (input != null) {
            std.debug.print("Failed to process input. Received stdIn aswell as input arg", .{});
            std.process.exit(1);
        } else {
            input = inputAsArg;
        }
    }
    if (input != null) {
        std.debug.print("Value of input (size: {d}): {?s}\n", .{ input.?.len, input });
    } else {
        std.debug.print("No input provided\n", .{});
    }
    std.debug.print("Overview of set options: \n excludes: {s}\n", .{excludes_comma_separated_string});
}

fn setExcludes(parameter: ?[]const u8) void {
    if (parameter == null) {
        std.debug.print("Failed to setExcludes.\n", .{});
        std.process.exit(1);
    }
    excludes_comma_separated_string = parameter.?;
}
