# Workshop Testing in Rust

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
- Test isolation
  - Isolating external dependencies
  - Isolating the filesystem
- Tips and tricks
  - Testing panics `#[should_panic]`
  - `#[ignore]` attribute
  - `#[cfg(test)]` for convenience methodsd
  - Testing async code
- Exploratory testing
  - Property-based testing
  - Mutation testing
- Testing frameworks and tools
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
- Speeding up tests
  - Unit tests vs integration tests
  - Faster integration tests: all in one file
  - Parallel and sequential tests
- Test coverage
- Benchmark tests
- Structuring code for better testing
  Onion Architecture / Hexagonal Architecture
  




