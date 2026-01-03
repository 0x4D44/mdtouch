use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::ExitCode;

use filetime::{set_file_times, FileTime};

/// Compile-time build date and time. To override, set the BUILD_DATETIME environment
/// variable at compile time. Otherwise, a default value is used.
const BUILD_DATETIME: &str = match option_env!("BUILD_DATETIME") {
    Some(val) => val,
    None => "2025-02-03 10:00:00",
};

/// Returns a detailed help message describing the usage of the tool.
fn help_message() -> String {
    let mut msg = String::new();
    msg.push_str("Usage: mdtouch [OPTIONS] <file> [file...]\n\n");
    msg.push_str(
        "A command line tool to mimic the behaviour of the Unix touch command on Windows.\n",
    );
    msg.push_str(
        "If the file does not exist, it will be created. Otherwise, its access and modification\n",
    );
    msg.push_str("times will be updated to the current time.\n\n");
    msg.push_str("Options:\n");
    msg.push_str("  -h, -?      Display this help message and exit.\n");
    msg
}

/// Touches a file at the given path, mimicking the behaviour of the Unix `touch` command.
/// If the file does not exist, it is created. In either case, the file's access and
/// modification times are updated to the current time.
fn touch_file<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    let path = path.as_ref();
    if !path.exists() {
        // Create the file if it does not exist.
        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
    }
    // Update the file's access and modification times to now.
    let now = FileTime::now();
    set_file_times(path, now, now)
}

/// Runs the application logic.
///
/// # Arguments
///
/// * `args` - A vector of command line arguments (excluding the program name).
/// * `writer` - A mutable reference to a writer for standard output.
fn run<W: Write>(args: Vec<String>, mut writer: W) -> std::io::Result<()> {
    // If no arguments are provided, print the version and a short summary.
    if args.is_empty() {
        writeln!(writer, "mdtouch  {}", BUILD_DATETIME)?;
        writeln!(
            writer,
            "A tool to update file timestamps or create empty files, mimicking the Unix touch command."
        )?;
        return Ok(());
    }

    // If any argument is a help flag, display help and exit.
    if args.iter().any(|arg| arg == "-h" || arg == "-?") {
        writeln!(writer, "{}", help_message())?;
        return Ok(());
    }

    // Process each file argument.
    for filename in args {
        if let Err(e) = touch_file(&filename) {
            // In the main loop, we print to stderr usually, but here we propagate the error
            // so main can handle it.
            // However, to mimic the original behavior of printing "Error touching ...",
            // we will format the error into a new Error.
            return Err(std::io::Error::other(format!(
                "Error touching {}: {}",
                filename, e
            )));
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();

    if let Err(e) = run(args, std::io::stdout()) {
        eprintln!("{}", e);
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::path::PathBuf;
    use std::thread;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    /// Generates a unique temporary file path in the system's temporary directory.
    fn unique_temp_file() -> PathBuf {
        let mut path = env::temp_dir();
        // Use the current time in nanoseconds to generate a unique file name.
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        path.push(format!("mdtouch_test_{}.tmp", nanos));
        path
    }

    #[test]
    fn test_touch_new_file() {
        let path = unique_temp_file();
        // Ensure the file does not exist.
        if path.exists() {
            fs::remove_file(&path).expect("Failed to remove pre-existing test file.");
        }
        assert!(
            !path.exists(),
            "Test file should not exist before touching."
        );

        touch_file(&path).expect("Failed to touch new file.");

        assert!(path.exists(), "File should exist after touching.");

        // Clean up
        fs::remove_file(&path).expect("Failed to remove test file.");
    }

    #[test]
    fn test_touch_existing_file_updates_modification_time() {
        let path = unique_temp_file();
        // Create the file initially with some content.
        fs::write(&path, b"initial content").expect("Failed to create test file.");

        // Set the modification time to a fixed point in the past.
        let past = FileTime::from_unix_time(1_000_000, 0); // Arbitrary time in the past.
        set_file_times(&path, past, past).expect("Failed to set file times.");

        let metadata_before = fs::metadata(&path).expect("Failed to get metadata.");
        let mod_time_before = metadata_before
            .modified()
            .expect("Failed to get modified time.");

        // Sleep briefly to ensure that the system clock advances.
        thread::sleep(Duration::from_secs(1));

        touch_file(&path).expect("Failed to touch existing file.");

        let metadata_after = fs::metadata(&path).expect("Failed to get metadata.");
        let mod_time_after = metadata_after
            .modified()
            .expect("Failed to get modified time.");

        assert!(
            mod_time_after > mod_time_before,
            "Modification time should be updated."
        );

        // Clean up
        fs::remove_file(&path).expect("Failed to remove test file.");
    }

    #[test]
    fn test_help_message_contains_usage() {
        let help = help_message();
        assert!(
            help.contains("Usage:"),
            "Help message should contain 'Usage:'"
        );
        assert!(
            help.contains("-h"),
            "Help message should mention '-h' option"
        );
        assert!(
            help.contains("-?"),
            "Help message should mention '-?' option"
        );
    }

    #[test]
    fn test_run_no_args() {
        let mut output = Vec::new();
        let result = run(vec![], &mut output);
        assert!(result.is_ok());
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("mdtouch"));
        assert!(output_str.contains("A tool to update file timestamps"));
    }

    #[test]
    fn test_run_help_arg() {
        let mut output = Vec::new();
        let result = run(vec!["-h".to_string()], &mut output);
        assert!(result.is_ok());
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("Usage:"));
    }

    #[test]
    fn test_run_touch_file() {
        let path = unique_temp_file();
        let path_str = path.to_str().unwrap().to_string();
        let mut output = Vec::new();

        let result = run(vec![path_str], &mut output);
        assert!(result.is_ok());
        assert!(path.exists());

        // Cleanup
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_run_error_handling() {
        // We use a directory path which cannot be created as a file
        // path variable removed as it was unused
        let mut output = Vec::new();

        // Trying to 'touch' an existing directory typically updates its timestamp on Unix,
        // but OpenOptions(...).create(true).write(true).open(dir) fails on Windows with "Access is denied"
        // or "is a directory" depending on the OS.
        // Let's force an error by using a path with a non-existent parent directory.
        // E.g. temp_dir / "non_existent_dir" / "file.txt"

        let mut bad_path = env::temp_dir();
        bad_path.push("non_existent_dir_xyz_123");
        bad_path.push("file.txt");

        let bad_path_str = bad_path.to_str().unwrap().to_string();

        let result = run(vec![bad_path_str], &mut output);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Error touching"));
    }
}
