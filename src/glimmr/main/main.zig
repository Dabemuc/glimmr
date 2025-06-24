const std = @import("std");
const core = @import("cli_core");
const builtin = @import("builtin");
const buildCli = @import("buildCli.zig").buildCli;

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    var cli_helper = core.CliHelper.init(allocator);
    defer cli_helper.deinit();

    if (builtin.mode == .Debug) {
        std.debug.print("== Running Glimmr in debug mode! ==\n", .{});
        cli_helper.enableDebugLogs();
    }

    try buildCli(&cli_helper, allocator);
}
