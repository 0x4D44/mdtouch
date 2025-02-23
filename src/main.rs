use std::env;
use std::fs::OpenOptions;
use std::path::Path;
use std::process;

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
    msg.push_str("A command line tool to mimic the behaviour of the Unix touch command on Windows.\n");
    msg.push_str("If the file does not exist, it will be created. Otherwise, its access and modification\n");
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
        OpenOptions::new().create(true).write(true).open(path)?;
    }
    // Update the file's access and modification times to now.
    let now = FileTime::now();
    set_file_times(path, now, now)
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    // If no arguments are provided, print the version and a short summary.
    if args.is_empty() {
        println!("mdtouch  {}", BUILD_DATETIME);
        println!("A tool to update file timestamps or create empty files, mimicking the Unix touch command.");
        return;
    }

    // If any argument is a help flag, display help and exit.
    if args.iter().any(|arg| arg == "-h" || arg == "-?") {
        println!("{}", help_message());
        return;
    }

    // Process each file argument.
    for filename in args {
        if let Err(e) = touch_file(&filename) {
            eprintln!("Error touching {}: {}", filename, e);
            process::exit(1);
        }
    }
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
        assert!(help.contains("Usage:"), "Help message should contain 'Usage:'");
        assert!(help.contains("-h"), "Help message should mention '-h' option");
        assert!(help.contains("-?"), "Help message should mention '-?' option");
    }
}
