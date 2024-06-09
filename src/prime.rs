pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        let primes = [2, 3, 5, 7];

        for n in primes.into_iter() {
            assert!(is_prime(n));
        }
    }

    #[test]
    fn test_is_not_prime() {
        let not_primes = [0, 4, 6, 8, 9, 10];

        for n in not_primes.into_iter() {
            assert!(!is_prime(n));
        }
    }
}
