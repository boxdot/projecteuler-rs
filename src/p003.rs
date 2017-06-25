// What is the largest prime factor of the number 600851475143 ?

/// return all non-even primes less `limit`
pub fn sieve_of_eratosthenes(limit: u64) -> Vec<u64> {

    let mut sieve: Vec<bool> = vec![true; (limit as f64 / 2f64).ceil() as usize - 1];
    // 0 -> 3
    // 1 -> 5
    // 2 -> 7
    // 3 -> 9 etc...

    let num_to_pos = |n| (n as usize - 3) / 2;
    let pos_to_num = |pos| (2 * (pos as u64) + 3);

    let max = (limit as f64).sqrt() as u64;
    let mut n = 3;
    while n <= max {
        if sieve[num_to_pos(n)] {
            let mut i = n;
            while i <= limit / n {
                // println!("pos {} -> {} * {}", num_to_pos(n * i), n, i);
                sieve[num_to_pos(n * i)] = false;
                i += 2;
            }
        }
        n += 2;
    }

    sieve
        .iter()
        .enumerate()
        .filter_map(|(pos, &is_prime)| {
            // println!("pos {} -> {} ==> {}", pos, pos_to_num(pos), is_prime);
            if is_prime {
                Some(pos_to_num(pos))
            } else {
                None
            }
        })
        .collect()
}

fn largest_prime_factor(n: u64) -> u64 {
    if n == 2 {
        return 2;
    }

    let n = if n % 2 == 0 { n / 2 } else { n };
    let primes = sieve_of_eratosthenes((n as f64).sqrt() as u64);
    for &p in primes.iter().rev() {
        if n % p == 0 {
            return p;
        }
    }
    assert!(false);
    0
}

pub fn solve() -> u64 {
    largest_prime_factor(600_851_475_143)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sieve_of_eratosthenes() {
        assert!(sieve_of_eratosthenes(2) == vec![]);
        assert!(sieve_of_eratosthenes(3) == vec![3]);
        assert!(sieve_of_eratosthenes(5) == vec![3, 5]);
        assert!(
            sieve_of_eratosthenes(50) == vec![3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47]
        );
        assert!(
            sieve_of_eratosthenes(51) == vec![3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47]
        );
    }

    #[test]
    fn test_largest_prime_factor() {
        assert!(largest_prime_factor(2) == 2);
        assert!(largest_prime_factor(3) == 3);
        assert!(largest_prime_factor(15) == 5);
        assert!(largest_prime_factor(100) == 5);
        assert!(largest_prime_factor(2 * 3 * 5 * 53) == 53);
    }
}
