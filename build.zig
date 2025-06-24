const std = @import("std");

pub fn build(b: *std.Build) void {

    // Build
    const target: std.Build.ResolvedTarget = b.standardTargetOptions(.{});
    const optimize: std.builtin.OptimizeMode = .Debug; // TODO: Change to ReleaseSafe

    const util = b.addModule("util", .{
        .root_source_file = b.path("src/util/main/root.zig"),
        .target = target,
        .optimize = optimize,
    });

    const cli_core = b.addModule("cli_core", .{
        .root_source_file = b.path("src/cli_core/main/root.zig"),
        .imports = &.{
            .{ .name = "util", .module = util },
        },
        .target = target,
        .optimize = optimize,
    });

    const glimmr = b.createModule(.{
        .root_source_file = b.path("src/glimmr/main/main.zig"),
        .imports = &.{
            .{ .name = "cli_core", .module = cli_core },
            .{ .name = "util", .module = util },
        },
        .target = target,
        .optimize = optimize,
    });

    const exe = b.addExecutable(.{
        .name = "glimmr",
        .root_module = glimmr,
    });

    b.installArtifact(exe);

    // Run step
    const run_cmd = b.addRunArtifact(exe);
    if (b.args) |args| run_cmd.addArgs(args);
    b.step("run", "Build and run the app").dependOn(&run_cmd.step);

    // Tests
    // Util tests
    const util_test = b.addTest(.{
        .root_source_file = b.path("src/util/test/test.zig"),
        .target = target,
        .optimize = .Debug,
    });
    const run_util_tests = b.addRunArtifact(util_test);
    b.step("test-util", "Run util tests").dependOn(&run_util_tests.step);
    // Core tests
    const cli_core_test = b.addTest(.{
        .root_source_file = b.path("src/cli_core/test/test.zig"),
        .target = target,
        .optimize = .Debug,
    });
    const run_cli_core_tests = b.addRunArtifact(cli_core_test);
    b.step("test-cli_core", "Run cli_core tests").dependOn(&run_cli_core_tests.step);

    // Glimmr tests
    const glimmr_test = b.addTest(.{
        .root_source_file = b.path("src/glimmr/test/test.zig"),
        .target = target,
        .optimize = .Debug,
    });
    const run_glimmr_tests = b.addRunArtifact(glimmr_test);
    b.step("test-glimmr", "Run glimmr tests").dependOn(&run_glimmr_tests.step);

    // Run all tests
    const test_all = b.step("test", "Run all tests");
    test_all.dependOn(&run_util_tests.step);
    test_all.dependOn(&run_cli_core_tests.step);
    test_all.dependOn(&run_glimmr_tests.step);
}
