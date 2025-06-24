const std = @import("std");
const logging = @import("util - cli_util").logging;

const logger = logging.Logger.init("util");

pub const Cli = struct {
    // Data fields
    excludes: std.ArrayList([]const u8),
    input: ?[]const u8,

    // callback functions
    setExcludes: fn (self: *Cli, parameter: ?[]const u8) void,

    // init & deinit
    pub fn init(allocator: std.mem.Allocator) Cli {
        logger.logDebug("Initializing Cli object.", .{});
        return Cli{
            .excludes = std.ArrayList([]const u8).init(allocator),
            .input = null,
            .setExcludes = setExcludesImpl,
        };
    }

    pub fn deinit(self: *Cli) void {
        logger.logDebug("Deinitializing Cli object.", .{});
        for (self.excludes.items) |item| {
            self.excludes.allocator.free(item);
        }
        self.excludes.deinit();
    }
};

fn setExcludesImpl(self: *Cli, parameter: ?[]const u8) void {
    const param = parameter orelse {
        logger.logError("Error: --exclude option requires a parameter.\n", .{});
        std.process.exit(1);
    };

    logger.logDebug("Parsing excludes: {s}", .{parameter});

    const allocator = self.excludes.allocator;

    // Split string by ','
    var it = std.mem.splitScalar(u8, param, ',');
    while (it.next()) |item| {
        // Skip empty items
        if (item.len == 0) continue;

        // We must 'dupe' the slice. The 'parameter' slice is likely temporary
        // and owned by the CLI parser. Our ArrayList needs to own its own copies.
        const owned_item = allocator.dupe(u8, item) catch |err| {
            logger.logError("Failed to allocate memory for exclude item: {s}\n", .{item});
            logger.logError("{any}\n", .{err});
            std.process.exit(1);
        };

        // Add item to exclude list
        self.excludes.append(owned_item) catch |err| {
            logger.logError("Failed to append exclude item to list: {s}\n", .{item});
            logger.logError("{any}\n", .{err});
            std.process.exit(1);
        };
    }
}
