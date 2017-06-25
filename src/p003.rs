// What is the largest prime factor of the number 600851475143 ?

fn largest_prime_factor(n: u64) -> u64 {
    let mut n = n;

    let mut prev_factor = 1;
    if n % 2 == 0 {
        n = n / 2;
        while n % 2 == 0 {
            n = n / 2;
        }
        prev_factor = 2
    }

    let mut factor = 3;
    let mut max_factor = (n as f64).sqrt() as u64;
    while n > 1 && factor <= max_factor {
        if n % factor == 0 {
            prev_factor = factor;
            n = n / factor;
            while n % factor == 0 {
                n = n / factor;
            }
            max_factor = (n as f64).sqrt() as u64;
        }
        factor += 1;
    }

    if n == 1 { prev_factor } else { n }
}

pub fn solve() -> u64 {
    largest_prime_factor(600_851_475_143)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_largest_prime_factor() {
        assert!(largest_prime_factor(2) == 2);
        assert!(largest_prime_factor(3) == 3);
        assert!(largest_prime_factor(15) == 5);
        assert!(largest_prime_factor(100) == 5);
        assert!(largest_prime_factor(2 * 3 * 5 * 53) == 53);
    }
}
