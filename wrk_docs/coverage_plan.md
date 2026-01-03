# Coverage Improvement Plan

## Goal
Increase code coverage from ~70% to >98% by testing the CLI logic currently residing in `main`.

## Strategy
Refactor the application to decouple the CLI logic from the standard output and process exit, allowing it to be unit tested.

## Steps

### Stage 1: Refactor `main` for Testability
1.  **Extract Logic:** Move the body of `main` into a new function `run` (or similar).
2.  **Dependency Injection:** Modify `run` to accept:
    *   Arguments (e.g., `Vec<String>`).
    *   Output writer (e.g., `&mut impl std::io::Write`) to capture `println!` output.
3.  **Error Handling:** Change `run` to return a `Result` instead of calling `process::exit`.
4.  **Update `main`:** `main` will simply collect args, call `run` with `stdout`, and handle the final `Result`.

### Stage 2: Add Unit Tests for CLI Logic
1.  **Test No Arguments:** Verify that passing an empty argument list writes the version/summary to the injected writer.
2.  **Test Help Flag:** Verify that passing `-h` or `-?` writes the help message to the injected writer.
3.  **Test File Execution:**
    *   Create a temporary directory/file.
    *   Call `run` with the file path.
    *   Verify the file is created/updated.
4.  **Test Error Handling:**
    *   Pass an invalid path (e.g., a directory as a file, or a path in a non-existent directory if strict).
    *   Verify `run` returns an error.

### Stage 3: Verification
1.  Run `cargo llvm-cov` to confirm coverage has increased.
2.  Ensure all tests pass.
