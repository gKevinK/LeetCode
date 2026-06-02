impl Solution {
    pub fn sum_of_largest_primes(s: String) -> i64 {
        let input = s.bytes().map(|b| (b - b'0') as i64).collect::<Vec<_>>();
        let mut s = [0, 0, 0];
        for i in 0..input.len() {
            let mut t = 0;
            for j in i..input.len() {
                t = t * 10 + input[j];
                if t != s[0] && t != s[1] && t > s[2] && Self::is_prime(t) {
                    if t > s[0] {
                        s = [t, s[0], s[1]];
                    } else if t > s[1] {
                        s = [s[0], t, s[1]];
                    } else {
                        s[2] = t;
                    }
                }
            }
        }
        s.iter().sum()
    }

    fn is_prime(n: i64) -> bool {
        if n <= 1 {
            return false;
        }
        if n == 2 || n == 3 {
            return true;
        }
        if n % 2 == 0 || n % 3 == 0 {
            return false;
        }
        let mut i = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }
}