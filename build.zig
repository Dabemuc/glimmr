const std = @import("std");

pub fn build(b: *std.Build) void {

    // Build
    const target: std.Build.ResolvedTarget = b.standardTargetOptions(.{});
    const optimize: std.builtin.OptimizeMode = .Debug; // TODO: Change to ReleaseSafe

    const cli_core = b.addModule("cli_core", .{
        .root_source_file = b.path("src/cli_core/main/root.zig"),
        .target = target,
        .optimize = optimize,
    });

    const glimmr = b.createModule(.{
        .root_source_file = b.path("src/glimmr/main/main.zig"),
        .imports = &.{
            .{ .name = "cli_core", .module = cli_core },
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
    // Core tests
    const core_test = b.addTest(.{
        .root_source_file = b.path("src/cli_core/test/test.zig"),
        .target = target,
        .optimize = .Debug,
    });
    const run_core_tests = b.addRunArtifact(core_test);
    b.step("test-core", "Run cli_core tests").dependOn(&run_core_tests.step);

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
    test_all.dependOn(&run_core_tests.step);
    test_all.dependOn(&run_glimmr_tests.step);
}
