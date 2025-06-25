const std = @import("std");

const posix = std.posix;
const c = @import("std").c;

const util = @import("util");
const Cli = util.cli_util.Cli;
const logging = util.logging;

/// A struct for generating a cli interface by registering options and commands.
/// Enables parsing stdIn and process args to command, input and options.
/// Provides further utility like automatic help page generation.
pub const CliHelper = struct {
    logger: logging.Logger,

    registered_options: std.ArrayList(Option),
    // registered_commands: std.ArrayList(Command),

    pub fn init(allocator: std.mem.Allocator) CliHelper {
        var logger = logging.Logger.init("cli_core");
        logger.logDebug("Initializing CliHelper object.", .{});
        return CliHelper{
            .logger = logger,
            .registered_options = std.ArrayList(Option).init(allocator),
        };
    }

    pub fn deinit(self: *CliHelper) void {
        self.logger.logDebug("Deinitializing CliHelper object.", .{});
        self.registered_options.deinit();
        self.logger.deinit();
    }

    pub fn enableDebugLogs(self: *CliHelper) void {
        self.logger.setLogLevel(logging.LOG_LEVEL.DEBUG);
    }

    pub fn registerOption(self: *CliHelper, arg: Option) void {
        self.registered_options.append(arg) catch {
            self.logger.logError("Failed to register option {s}. Did you call init?\n", .{arg.long_name});
            std.process.exit(1);
        };
        self.logger.logDebug("Successfully registered Option '{s}'", .{arg.long_name});
    }

    pub fn parseArgs(self: *CliHelper, input_args_iter: std.process.ArgIterator) !?[]const u8 {
        var it = input_args_iter;
        // Skip first cause it is call of cli
        _ = it.next();
        var input: ?[]const u8 = null;
        var option_expecting_parameter: ?Option = null;
        while (true) {
            const arg_optional = it.next();
            if (arg_optional == null) break;
            const arg = arg_optional.?;
            self.logger.logDebug("Processing arg: {s}\n", .{arg});
            // Process arg
            if (arg[0] == '-') {
                // option
                self.logger.logDebug("Option received.\n", .{});
                const parsedOption = parseOption(self, arg[1..arg.len]);
                self.logger.logDebug("Recognized Option: '{s}'\n", .{parsedOption.long_name});
                // check if option expects parameter
                if (parsedOption.expects_parameter) {
                    self.logger.logDebug("Expecting parameter as next arg. Delaying calling callback until parameter received.\n", .{});
                    if (option_expecting_parameter != null) {
                        self.logger.logError("Option is missing a parameter: {?s}(-{?c})\n", .{ option_expecting_parameter.?.long_name, option_expecting_parameter.?.short_name });
                        std.process.exit(1);
                    }
                    option_expecting_parameter = parsedOption;
                } else {
                    self.logger.logDebug("Option does not expect parameter. Calling callback.\n", .{});
                    // Pass the context, and null for the parameter
                    parsedOption.callback(parsedOption.context, null);
                }
            } else {
                // non-option
                self.logger.logDebug("Input or Parameter received.\n", .{});
                // Check if a parameter is expected
                if (option_expecting_parameter != null) {
                    self.logger.logDebug("An option is waiting for a parameter. Calling callback with parameter.\n", .{});
                    // Pass the context and the argument value
                    const opt = option_expecting_parameter.?;
                    opt.callback(opt.context, arg);
                    option_expecting_parameter = null;
                } else if (input == null) {
                    self.logger.logDebug("Recognized as Input.\n", .{});
                    input = arg;
                } else {
                    self.logger.logError("Multiple inputs received as args. [{?s}, {s}]\n", .{ input, arg });
                    std.process.exit(1);
                }
            }
        }
        if (option_expecting_parameter != null) {
            self.logger.logError("Option is missing a parameter: {?s}(-{?c})\n", .{ option_expecting_parameter.?.long_name, option_expecting_parameter.?.short_name });
            std.process.exit(1);
        }
        return input;
    }

    fn parseOption(self: *CliHelper, option_name_to_parse: []const u8) Option {
        const index_of_option = for (self.registered_options.items, 0..) |raw_option, index| {
            const option: Option = raw_option;
            if (std.mem.eql(u8, option.long_name, option_name_to_parse)) break index;
            if (option.short_name) |short_name| {
                if (option_name_to_parse.len == 1 and option_name_to_parse[0] == short_name) break index;
            }
        } else null;

        if (index_of_option == null) {
            self.logger.logError("Option does not exist: {s}\n", .{option_name_to_parse});
            std.process.exit(1);
        }

        return self.registered_options.items[index_of_option.?];
    }

    pub fn readStdIn(_: *CliHelper, stdIn: std.fs.File, allocator: std.mem.Allocator) !?[]u8 {
        if (!try stdinHasData(stdIn)) {
            return null;
        }

        var buffer = try allocator.alloc(u8, 1024);
        const reader = stdIn.reader();
        const readSize = try reader.readAll(buffer);
        return buffer[0..readSize];
    }

    fn stdinHasData(stdIn: std.fs.File) !bool {
        const target = @import("builtin").target;
        if (target.os.tag == .windows) {
            return try windowsStdinHasData(stdIn);
        } else {
            return try unixStdinHasData(stdIn);
        }
    }

    // TODO: Find a working solution
    fn unixStdinHasData(_: std.fs.File) !bool {
        // const os = std.os;
        // const fd = stdIn.handle;
        //
        // var read_fds = os.linux.fd_set{};
        // os.linux.FD_ZERO(&read_fds);
        // os.linux.FD_SET(fd, &read_fds);
        //
        // var timeout = os.linux.timeval{
        //     .tv_sec = 0,
        //     .tv_usec = 0,
        // };
        //
        // const result = os.linux.select(fd + 1, &read_fds, null, null, &timeout);
        // if (result < 0) return error.SelectFailed;
        // return result > 0;
        return false;
    }

    fn windowsStdinHasData(stdIn: std.fs.File) !bool {
        const os = std.os.windows;
        const handle = stdIn.handle;

        var bytes_available: u32 = 0;
        const success = os.kernel32.PeekNamedPipe(
            handle,
            null,
            0,
            null,
            &bytes_available,
            null,
        );

        if (success == 0) return error.PeekNamedPipeFailed;
        return bytes_available > 0;
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
    callback: *const fn (context: ?*anyopaque, arg_value: ?[]const u8) void,
    context: ?*anyopaque, // This field will store the pointer to our `Cli` instance
    expects_parameter: bool,
};
