# Workshop Testing in Rust

![Course Banner](/assets/banner.png)

This is a workshop about testing in Rust. It is intended for people who are already a bit familiar with Rust and want to learn more about testing in Rust.

## Prerequisites

- Basic knowledge of Rust.
- A working Rust installation. (See https://rustup.rs/)
- A working Rust IDE (e.g. Visual Studio Code with the Rust extension)
  or an editor.

## Slides

Open the [slides](/slides.pdf) to follow along with the workshop.

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
- Test coverage: `cargo tarpaulin --out Html`

## Running the tests

If you want to run unit tests only, you can use the `--lib` flag:

```sh
cargo test --lib
```

To run only the integration tests, you can use the `--test` flag:

```sh
cargo test --test '*'
```

To run a specific test, you can use the `--test` flag with the test name:

```sh
cargo test --test fibonacci_is_prime
```

## Notable Frameworks and Libraries

### Test Runners

- [cargo-nextest](https://nexte.st/): A faster test runner for Rust.

### Parameterized Testing

- [rstest](https://github.com/la10736/rstest): Write parameterized tests in Rust.
- [test-case](https://github.com/frondeus/test-case): Another parameterized testing library.

### Property-Based Testing

- [quickcheck](https://github.com/BurntSushi/quickcheck): Automated property-based testing with shrinking support.
- [proptest](https://github.com/proptest-rs/proptest): Another property-based testing library.

### Mocking and Test Doubles

- [mockall](https://github.com/asomers/mockall): Trait-based mocking library.
- [mockito](https://github.com/lipanski/mockito): HTTP mocking library.
- [httpmock](https://github.com/alexliesenfeld/httpmock): Another HTTP mocking library.

### Testing Command Line Applications

- [assert_cmd](https://github.com/assert-rs/assert_cmd): Test command line applications.

### Filesystem and Network Isolation

- [tempfile](https://github.com/Stebalien/tempfile): Create temporary files and directories.

### Testing Snapshots

- [insta](https://github.com/mitsuhiko/insta?tab=readme-ov-file): Snapshot testing library.

### Faking Data

- [fakeit](https://crates.io/crates/fakeit): Fake data generator library with 130+ functions.

### Test Context Management

- [test-context](https://crates.io/crates/test-context): A library for writing tests with setup and teardown functions.

### Little Helpers

- [pretty_assertions](https://github.com/rust-pretty-assertions/rust-pretty-assertions): Alternative to `assert_eq!` with better diffing and color support.
- [assertor](https://github.com/google/assertor): Fluent assertion library for Rust with readable messages.

### Procedural Macro Testing

- [trybuild](https://crates.io/crates/trybuild): Testing procedural macros.