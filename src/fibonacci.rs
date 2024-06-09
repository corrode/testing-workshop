pub fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    for _ in 2..=n {
        c = a + b;
        a = b;
        b = c;
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(6), 8);
        assert_eq!(fibonacci(7), 13);
        assert_eq!(fibonacci(8), 21);
        assert_eq!(fibonacci(9), 34);
        assert_eq!(fibonacci(10), 55);
    }
}
