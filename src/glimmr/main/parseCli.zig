const std = @import("std");
const builtin = @import("builtin");
const CliHelper = @import("cli_core").CliHelper;
const Cli = @import("util").cli_util.Cli;
const Logger = @import("util").logging.Logger;

pub fn parseCli(cli_helper: *CliHelper, parsedCli: *Cli, allocator: std.mem.Allocator, logger: *Logger) !void {
    cli_helper.registerOption(.{
        .long_name = "exclude",
        .short_name = 'x',
        .description = "The files or directories to exclude as comma-separated string. (Example: '-x file1,folder1,./folder2/file2')",
        .callback = parsedCli.setExcludes,
        .context = parsedCli,
        .expects_parameter = true,
    });

    const stdIn = cli_helper.readStdIn(std.io.getStdIn(), allocator) catch |err| {
        logger.logError("Failed to read stdIn\n{}", .{err});
        std.process.exit(1);
    };
    if (stdIn != null) {
        parsedCli.input = stdIn;
    }
    const inputAsArg = cli_helper.parseArgs(std.process.args()) catch |err| {
        logger.logError("Failed to parse args\n{}", .{err});
        std.process.exit(1);
    };
    if (inputAsArg != null) {
        if (parsedCli.input != null) {
            logger.logError("Failed to process input. Received stdIn aswell as input arg", .{});
            std.process.exit(1);
        } else {
            parsedCli.input = inputAsArg;
        }
    }
    const inputOpt = parsedCli.input;
    if (inputOpt) |input| {
        logger.logDebug("Value of input (size: {d}): {s}\n", .{ input.len, input });
    } else {
        logger.logDebug("No input provided\n", .{});
    }
    // You are trying to print an ArrayList struct, you probably want to print its items
    // std.debug.print("Overview of set options:\n", .{});
    for (parsedCli.excludes.items) |item| {
        logger.logDebug("  exclude: {s}\n", .{item});
    }
}
