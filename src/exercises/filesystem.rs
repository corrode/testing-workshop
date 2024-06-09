//! Advanced testing topics

// Mocking the file system
#[cfg(test)]
mod tests {
    use std::fs;
    use std::io;
    use std::path::Path;

    // The following test accesses the file system.
    // This is not a good practice because it can impact other tests and cause side effects.
    // Furthermore, it pollutes the local file system with test files.
    //
    // Change the test to use the [`tempfile`](https://docs.rs/tempfile/latest/tempfile/) crate
    // to create a temporary directory and put the test files there. This way, the test is isolated
    // and can be run in parallel with other tests.
    #[test]
    fn test_file_system() -> io::Result<()> {
        let path = Path::new("hello.txt");
        fs::write(path, "Hello, world!")?;
        assert_eq!(fs::read_to_string(path)?, "Hello, world!");

        let path2 = Path::new("goodbye.txt");
        fs::write(path2, "Goodbye, world!")?;
        assert_eq!(fs::read_to_string(path2)?, "Goodbye, world!");

        // Count the text files in the current directory.
        // Uses a simple string search for ".txt" in the file name.
        let entries = fs::read_dir(".")?;
        let text_files = entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_name().to_string_lossy().contains(".txt"))
            .count();

        assert_eq!(text_files, 2);

        fs::remove_file(path)?;
        fs::remove_file(path2)?;
        Ok(())
    }

    // The following test uses the `tempfile` crate to create a temporary directory and put the test
    // files there. This way, the test is isolated and can be run in parallel with other tests.
    #[test]
    fn test_file_system_with_tempfile() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("hello.txt");
        fs::write(&path, "Hello, world!")?;
        assert_eq!(fs::read_to_string(&path)?, "Hello, world!");

        let path2 = dir.path().join("goodbye.txt");
        fs::write(&path2, "Goodbye, world!")?;
        assert_eq!(fs::read_to_string(&path2)?, "Goodbye, world!");

        // Count the text files in the current directory.
        // Uses a simple string search for ".txt" in the file name.
        let entries = fs::read_dir(dir.path())?;
        let text_files = entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_name().to_string_lossy().contains(".txt"))
            .count();

        assert_eq!(text_files, 2);

        Ok(())
    }
}
