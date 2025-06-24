const std = @import("std");

const c = @cImport({
    @cInclude("time.h");
    @cInclude("sys/time.h");
});

pub const LOG_LEVEL = enum {
    DEBUG,
    INFO,
    ERROR,
    SILENT,
};

pub const Logger = struct {
    log_level: LOG_LEVEL = LOG_LEVEL.INFO,

    module_name: []const u8,

    pub fn init(module_name: []const u8) Logger {
        return Logger{
            .module_name = module_name,
        };
    }

    pub fn deinit(_: *Logger) void {}

    /// Log Level:
    /// DEBUG -> Everything gets logged
    /// INFO ->  (default) Only error and info get logged
    /// ERROR -> Only error gets logged
    /// SILET -> Nothig gets logged
    pub fn setLogLevel(self: *Logger, lvl: LOG_LEVEL) void {
        self.log_level = lvl;
    }

    pub fn logDebug(self: *Logger, comptime fmt: []const u8, args: anytype) void {
        if (self.log_level == LOG_LEVEL.DEBUG) {
            log(self, fmt, args, LOG_LEVEL.DEBUG);
        }
    }

    pub fn logInfo(self: *Logger, comptime fmt: []const u8, args: anytype) void {
        if (self.log_level == LOG_LEVEL.DEBUG or self.log_level == LOG_LEVEL.INFO) {
            log(self, fmt, args, LOG_LEVEL.INFO);
        }
    }

    pub fn logError(self: *Logger, comptime fmt: []const u8, args: anytype) void {
        if (self.log_level == LOG_LEVEL.DEBUG or self.log_level == LOG_LEVEL.INFO or self.log_level == LOG_LEVEL.ERROR) {
            log(self, fmt, args, LOG_LEVEL.ERROR);
        }
    }

    fn log(self: *Logger, comptime fmt: []const u8, args: anytype, lvl: LOG_LEVEL) void {
        printFmt("[{s}][{s}]({s}): ", .{ self.module_name, @tagName(lvl), getCurrentTimeString() });
        printFmt(fmt, args);
        if (!(fmt.len > 0 and fmt[fmt.len - 1] == '\n')) {
            print("\n");
        }
    }

    fn print(comptime string: []const u8) void {
        printFmt(string, .{});
    }

    fn printFmt(comptime fmt: []const u8, args: anytype) void {
        std.debug.print(fmt, args);
    }

    pub fn getCurrentTimeString() [32]u8 {
        var now: c.time_t = undefined;
        _ = c.time(&now);

        const tm_ptr = c.localtime(&now);
        if (tm_ptr == null) {
            return [_]u8{ 'E', 'R', 'R', ':', 'l', 'o', 'c', 'a', 'l', 't', 'i', 'm', 'e', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 };
        }

        var tv: c.struct_timeval = undefined;
        if (c.gettimeofday(&tv, null) != 0) {
            return [_]u8{ 'E', 'R', 'R', ':', 'g', 'e', 't', 't', 'i', 'm', 'e', 'o', 'f', 'd', 'a', 'y', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 };
        }

        const tm = tm_ptr.*;

        var buffer: [32]u8 = undefined;

        const year_i32 = @as(i32, tm.tm_year) + 1900;
        if (year_i32 < 0) @panic("Invalid year value");
        const year_u32: u32 = @intCast(year_i32);

        const month_i32 = @as(i32, tm.tm_mon) + 1;
        if (month_i32 < 0) @panic("Invalid month value");
        const month_u32: u32 = @intCast(month_i32);

        const day_i32 = @as(i32, tm.tm_mday);
        if (day_i32 < 0) @panic("Invalid day value");
        const day_u32: u32 = @intCast(day_i32);

        const hour_i32 = @as(i32, tm.tm_hour);
        if (hour_i32 < 0) @panic("Invalid hour value");
        const hour_u32: u32 = @intCast(hour_i32);

        const min_i32 = @as(i32, tm.tm_min);
        if (min_i32 < 0) @panic("Invalid minute value");
        const min_u32: u32 = @intCast(min_i32);

        const sec_i32 = @as(i32, tm.tm_sec);
        if (sec_i32 < 0) @panic("Invalid second value");
        const sec_u32: u32 = @intCast(sec_i32);

        const usec_i32 = @as(i32, tv.tv_usec);
        if (usec_i32 < 0) @panic("Negative microseconds not allowed");
        const millis_u32: u32 = @intCast(@divFloor(usec_i32, 1000));

        const slice = std.fmt.bufPrint(
            &buffer,
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
            .{
                year_u32,
                month_u32,
                day_u32,
                hour_u32,
                min_u32,
                sec_u32,
                millis_u32,
            },
        ) catch {
            @panic("bufPrint failed");
        };

        var result: [32]u8 = undefined;
        for (0..slice.len) |i| {
            result[i] = slice[i];
        }
        for (slice.len..result.len) |i| {
            result[i] = 0;
        }

        return result;
    }
};
