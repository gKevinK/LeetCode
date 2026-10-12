const LIMIT: usize = 50_001;

const PRIME: [bool; LIMIT] = {
    let mut p = [true; LIMIT];
    p[0] = false;
    p[1] = false;
    let mut i = 2;
    while i * i <= LIMIT {
        let mut j = i * i;
        while j < LIMIT {
            p[j] = false;
            j += i;
        }
        i += 1;
    }
    p
};

impl Solution {
    pub fn largest_prime(n: i32) -> i32 {
        let mut sums = vec![];
        let mut s = 0;
        for i in 2..LIMIT {
            if PRIME[i] {
                s += i as i32;
                if Self::is_prime(s) {
                    sums.push(s);
                }
            }
            if s > 500_000 || s > n {
                break;
            }
        }
        let x = sums.partition_point(|&s| s <= n);
        if x > 0 { sums[x - 1] } else { 0 }
    }

    fn is_prime(n: i32) -> bool {
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