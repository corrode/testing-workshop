# Workshop Testing in Rust

![Course Banner](/assets/banner.png)

This is a workshop about testing in Rust. It is intended for people who are already a bit familiar with Rust and want to learn more about testing in Rust.

## Prerequisites

- Basic knowledge of Rust.
- A working Rust installation. (See https://rustup.rs/)
- A working Rust IDE (e.g. Visual Studio Code with the Rust extension)
  or an editor.

## Slides

Please see the file `Rust Testing Workshop.pdf` in this repository.

## Exercises

Go to the `src/exercises` module and follow the instructions there.
You can start with `basic.rs` and then move on to more advanced exercises.

## Workshop Topics

This workshop is about testing in Rust. We will cover the following topics:

- Writing unit tests in the same file as the code
- Returning `Result` from tests
- Keep tests close to the code
- Writing integration tests
- Integration tests vs functional tests
- Doctests
- Test doubles
  - Mocks
  - Traits
- Tips and tricks
  - You can return `Result` from tests
  - `#[ignore]` attribute
  - Testing panics `#[should_panic]`
  - `#[cfg(test)]` for convenience methods
- Test isolation
  - Isolating the filesystem: `tempdir`
  - Isolating the network: HTTP mocking
  - Isolating external dependencies: `mockall`
- Speeding up tests
  - Unit tests over integration tests
  - Faster integration tests: use a single compilation unit
  - cargo test runs tests in parallel
  - cargo nextest
  - sequential tests with `--test-threads=1` or 
    https://crates.io/crates/serial_test
- Exploratory testing
  - Property-based testing
  - Mutation testing
- Benchmark tests
- Structuring code for better testing
  Onion Architecture / Hexagonal Architecture
- Test coverage

## Notable frameworks and libraries

- cargo nextest
- rstest
- https://github.com/frondeus/test-case
- mockall
- assert_cmd
- tempdir
- pretty_assertions
- insta (snapshot testing)
- quickcheck
- proptest
- mockito (HTTP mocking)
- https://github.com/alexliesenfeld/httpmock





