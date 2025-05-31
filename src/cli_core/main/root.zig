const std = @import("std");

/// A struct storing registered arguments and providing methods on them
pub const CliHelper = struct {
    registered_args: std.ArrayList(Arg),

    pub fn init(allocator: std.mem.Allocator) CliHelper {
        return CliHelper{
            .registered_args = std.ArrayList(Arg).init(allocator),
        };
    }

    pub fn deinit(self: *CliHelper) void {
        self.registered_args.deinit();
    }

    pub fn registerArg(self: *CliHelper, arg: Arg) void {
        self.registered_args.append(arg) catch {
            std.debug.print("Failed to register argument {s}. Did you call init?", .{arg.long_name});
            std.process.exit(1);
        };
    }

    pub fn parseInputArgs(_: *CliHelper, input_args_iter: std.process.ArgIterator) !void {
        var it = input_args_iter;
        while (true) {
            const arg = it.next();
            if (arg == null) break;
            std.debug.print("{s}\n", .{arg.?});
        }
    }

    pub fn parseStdIn(_: *CliHelper, stdIn: std.fs.File) !void {
        var input: [5]u8 = undefined;
        _ = try stdIn.reader().readUntilDelimiter(&input, '\n');
        std.debug.print("The user entered: {s}\n", .{input});
    }
};

/// A struct representing an argument that can be passed to the cli.
/// - long_name: The long version of the arg. E.g. verbose -> accepting --verbose
/// - short_name: Optional. The short version of the arg. E.g. v -> accepting -v
/// - description: The description of this arg shown when calling --help or -h
/// - callback: The function that is called with args value when arg is found in input
pub const Arg = struct {
    long_name: []const u8,
    short_name: ?u8,
    description: []const u8,
    callback: *const fn (arg_value: []const u8) void,
};
