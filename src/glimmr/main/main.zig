const std = @import("std");
const core = @import("cli_core");
const builtin = @import("builtin");
const parseCli = @import("parseCli.zig").parseCli;
const Cli = @import("util").cli_util.Cli;

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    var cli_helper = core.CliHelper.init(allocator);
    defer cli_helper.deinit();

    if (builtin.mode == .Debug) {
        std.debug.print("== Running Glimmr in debug mode! ==\n", .{});
        cli_helper.enableDebugLogs();
    }

    var parsedCli = Cli.init(allocator);
    defer parsedCli.deinit();

    try parseCli(&cli_helper, &parsedCli, allocator);
}
