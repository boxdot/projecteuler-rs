/// Upper bound of prime counting function π(x)
/// Modified from Robin 1983 for 6-39016
/// Cf. also https://stackoverflow.com/a/25440642/1868581
fn pi_upper_bound(x: u64) -> u64 {
    let x = x as f64;
    let lnx = x.ln();
    let ln2x = lnx * lnx;
    (x * (lnx + 0.6000 * ln2x)) as u64
}

/// Calculate nth-prime by using the sieve of Erastosthenes
pub fn nth_prime(n: u64) -> u64 {
    if n <= 10 {
        let first_primes: [u64; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        return first_primes[n as usize - 1];
    }

    let limit = pi_upper_bound(n);
    println!("limit {}", limit);

    let mut sieve: Vec<bool> = vec![true; (limit as f64 / 2f64).ceil() as usize - 1];
    // 0 -> 3
    // 1 -> 5
    // 2 -> 7
    // 3 -> 9 etc...

    let num_to_pos = |n| (n as usize - 3) / 2;

    let mut p = 3;
    let mut num = 2;
    while num < n {
        if sieve[num_to_pos(p)] {
            let mut i = p;
            while i < limit / p {
                sieve[num_to_pos(p * i)] = false;
                i += 2;
            }
            num += 1;
        }
        p += 2;
    }

    while !sieve[num_to_pos(p)] {
        p += 2
    }
    p
}

pub fn solve() -> u64 {
    nth_prime(10001)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_nth_prime() {
        assert_eq!(nth_prime(11), 31);
        assert_eq!(nth_prime(12), 37);
        assert_eq!(nth_prime(13), 41);
        assert_eq!(nth_prime(14), 43);
        assert_eq!(nth_prime(15), 47);
        assert_eq!(nth_prime(100), 541);
    }
}
