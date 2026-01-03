use std::env;
use std::process::Command;

#[test]
fn test_main_binary_runs() {
    // Attempt to locate the binary.
    // When running under cargo llvm-cov, the binary is in target/llvm-cov-target/debug/
    // The test binary is in target/llvm-cov-target/debug/deps/

    let mut bin_path = env::current_exe().expect("Failed to get current exe path");
    bin_path.pop(); // Remove the test executable name.

    // If we are in 'deps', go up one level.
    if bin_path.file_name().and_then(|s| s.to_str()) == Some("deps") {
        bin_path.pop();
    }

    bin_path.push("mdtouch.exe");

    if !bin_path.exists() {
        // Try without extension if on non-windows (though we are on win32)
        // Or maybe it's in a different place?
        // Let's print where we looked if we fail.
        eprintln!("Could not find binary at: {:?}", bin_path);

        // Fallback: try relative path from CWD (which is project root)
        // This is risky as target dir name changes with llvm-cov
        return;
    }

    // Run with -h
    let output = Command::new(&bin_path)
        .arg("-h")
        .output()
        .expect("Failed to execute binary");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Usage:"));

    // Run with no args
    let output_no_args = Command::new(&bin_path)
        .output()
        .expect("Failed to execute binary");

    assert!(output_no_args.status.success());
    let stdout_no_args = String::from_utf8(output_no_args.stdout).unwrap();
    assert!(stdout_no_args.contains("mdtouch"));

    // Run with invalid file to trigger error exit
    let mut bad_path = env::temp_dir();
    bad_path.push("non_existent_dir_xyz_123");
    bad_path.push("file.txt");

    let output_err = Command::new(&bin_path)
        .arg(bad_path)
        .output()
        .expect("Failed to execute binary");

    assert!(!output_err.status.success());
    let stderr = String::from_utf8(output_err.stderr).unwrap();
    assert!(stderr.contains("Error touching"));
}
