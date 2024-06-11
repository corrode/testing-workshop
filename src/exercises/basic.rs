//! Exercises for learning how to write tests in Rust.
//! Contains sample code to get you started.
//! Feel free to edit the code or the tests to practice your Rust skills.

/// Returns true if the given string is a palindrome.
fn is_palindrome(text: &str) -> bool {
    text.chars().eq(text.chars().rev())
}

/// Returns true if the given string is in alphabetical order.
fn is_in_alphabetical_order(text: &str) -> bool {
    text.chars().zip(text.chars().skip(1)).all(|(a, b)| a <= b)
}

/// Tries to convert a word to a number. Returns a `Result` with the number or an error message.
fn word_to_number(word: &str) -> Result<u64, String> {
    match word {
        "one" => Ok(1),
        "two" => Ok(2),
        "three" => Ok(3),
        "fourtytwo" => Ok(42),
        _ => Err(format!("Unknown number word: {}", word)),
    }
}

#[cfg(test)]
mod tests {
    // Test the `is_palindrome` function her.

    // Test the `is_in_alphabetical_order` function here.

    // Test the `word_to_number` function here.
    // Return a `Result` from the test function.

    // Below is a test that currently fails. Ignore it for now.
    #[test]
    #[ignore]
    fn test_fail() {
        let foo = "asdf";
        assert_eq!(1, 2, "This test should fail: {foo}");
    }

    // Below is a test which should panic. Test that it does.
    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("This test should panic");
    }

    // You can add additional functionality to a struct with `#[cfg(test)]`,
    // which is only compiled when running tests!
    struct Foo {
        x: i32,
    }

    // Normal implementation of Foo
    impl Foo {
        // ...
    }

    // Additional functionality for Foo only available in tests
    #[cfg(test)]
    impl Foo {
        fn random_number() -> i32 {
            42
        }

        fn special_foo_constructor() -> Foo {
            Foo { x: 42 }
        }
    }

    // These methos can now be used in tests. Play around with them!
    #[test]
    #[ignore]
    fn test_foo() {
        todo!()
    }
}
