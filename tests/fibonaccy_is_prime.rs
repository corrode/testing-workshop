use testing::fibonacci::fibonacci;
use testing::prime::is_prime;

#[test]
fn test_fibonacci_is_prime() {
    let fib_is_prime = [
        (0, 0, false),
        (1, 1, false),
        (2, 1, false),
        (3, 2, true),
        (4, 3, true),
        (5, 5, true),
        (6, 8, false),
        (7, 13, true),
        (8, 21, false),
        (9, 34, false),
        (10, 55, false),
    ];

    for (n, fib, prime) in fib_is_prime.into_iter() {
        assert_eq!(fibonacci(n), fib);
        assert_eq!(is_prime(fibonacci(n)), prime);
    }
}
