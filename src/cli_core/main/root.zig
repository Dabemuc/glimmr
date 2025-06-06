const std = @import("std");

/// A struct for generating a cli interface by registering options and commands.
/// Enables parsing stdIn and process args to command, input and options.
/// Provides further utility like automatic help page generation.
pub const CliHelper = struct {
    registered_options: std.ArrayList(Option),
    // registered_commands: std.ArrayList(Command),

    pub fn init(allocator: std.mem.Allocator) CliHelper {
        return CliHelper{
            .registered_options = std.ArrayList(Option).init(allocator),
        };
    }

    pub fn deinit(self: *CliHelper) void {
        self.registered_options.deinit();
    }

    pub fn registerOption(self: *CliHelper, arg: Option) void {
        self.registered_options.append(arg) catch {
            std.debug.print("Failed to register option {s}. Did you call init?", .{arg.long_name});
            std.process.exit(1);
        };
    }

    pub fn parseArgs(_: *CliHelper, input_args_iter: std.process.ArgIterator) !void {
        var it = input_args_iter;
        while (true) {
            const arg = it.next();
            if (arg == null) break;
            std.debug.print("{s}\n", .{arg.?});
        }
    }

    pub fn readStdIn(_: *CliHelper, stdIn: std.fs.File) ![]u8 {
        var buffer: [1024]u8 = undefined;
        const reader = stdIn.reader();
        const readSize = try reader.readAll(&buffer);
        std.debug.print("stdIn: {s}\n", .{buffer[0..readSize]});
        std.debug.print("stdIn Size: {}\n", .{readSize});
        return buffer[0..readSize];
    }

    pub fn printHelpPage() void {}
};

/// A struct representing an option that can be passed to the cli.
/// - long_name: The long version of the option. E.g. verbose -> accepting --verbose
/// - short_name: Optional. The short version of the option. E.g. v -> accepting -v
/// - description: The description of this option shown when calling --help or -h
/// - callback: The function that is called with options value when option is found in input
pub const Option = struct {
    long_name: []const u8,
    short_name: ?u8,
    description: []const u8,
    callback: *const fn (arg_value: []const u8) void,
};
