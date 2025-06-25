const std = @import("std");
const core = @import("cli_core");
const builtin = @import("builtin");
const parseCli = @import("parseCli.zig").parseCli;
const Cli = @import("util").cli_util.Cli;
const logging = @import("util").logging;

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    var cli_helper = core.CliHelper.init(allocator);
    defer cli_helper.deinit();

    var logger = logging.Logger.init("glimmr");

    if (builtin.mode == .Debug) {
        logger.logDebug("== Running Glimmr in debug mode! ==\n", .{});
    }

    logger.logDebug("Parsing cli arguments and input.", .{});
    var parsedCli = Cli.init(allocator);
    defer parsedCli.deinit();
    try parseCli(&cli_helper, &parsedCli, allocator, &logger);
}
