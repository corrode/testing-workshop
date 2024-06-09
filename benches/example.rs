fn main() {
    // Run registered benchmarks.
    divan::main();
}

// Register a recursive `fibonacci` function and benchmark it over multiple cases.
#[divan::bench(args = [1, 2, 4, 8, 16, 32])]
fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        fibonacci(n - 2) + fibonacci(n - 1)
    }
}

// Run our own, imperative `fibonacci` function and benchmark it over multiple cases.
use testing::fibonacci::fibonacci as fibonacci_imperative_impl;

#[divan::bench(args = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048])]
fn fibonacci_imperative(n: u64) -> u64 {
    fibonacci_imperative_impl(n)
}
