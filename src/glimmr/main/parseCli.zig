const std = @import("std");
const builtin = @import("builtin");
const CliHelper = @import("cli_core").CliHelper;
const Cli = @import("util").cli_util.Cli;

pub fn parseCli(cli_helper: *CliHelper, parsedCli: Cli, allocator: std.mem.Allocator) !void {
    cli_helper.registerOption(.{
        .long_name = "exclude",
        .short_name = 'x',
        .description = "The files or directories to exclude as comma-separated string. (Example: '-x file1,folder1,./folder2/file2')",
        .callback = parsedCli.setExcludes,
        .expects_parameter = true,
    });

    const stdIn = cli_helper.readStdIn(std.io.getStdIn(), allocator) catch |err| {
        std.debug.print("Failed to read stdIn\n{}", .{err});
        std.process.exit(1);
    };
    if (stdIn != null) {
        parsedCli.input = stdIn;
    }
    const inputAsArg = cli_helper.parseArgs(std.process.args()) catch |err| {
        std.debug.print("Failed to parse args\n{}", .{err});
        std.process.exit(1);
    };
    if (inputAsArg != null) {
        if (parsedCli.input != null) {
            std.debug.print("Failed to process input. Received stdIn aswell as input arg", .{});
            std.process.exit(1);
        } else {
            parsedCli.input = inputAsArg;
        }
    }
    if (parsedCli.input != null) {
        std.debug.print("Value of input (size: {d}): {?s}\n", .{ parsedCli.input.?.len, parsedCli.input });
    } else {
        std.debug.print("No input provided\n", .{});
    }
    std.debug.print("Overview of set options: \n excludes: {s}\n", .{parsedCli.excludes});
}
