//! Use `assert_cmd` to test the CLI
//!
//! For example, try to write a test for the `ls` command
//! See https://doc.rust-lang.org/nightly/std/process/struct.Command.html

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    // This might come in handy...
    use predicates::prelude::*;

    #[test]
    fn test_ls_command() {
        let mut list_dir = Command::new("ls");

        // Write some assertions here
    }
}
