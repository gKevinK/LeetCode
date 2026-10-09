impl Solution {
    pub fn complete_prime(num: i32) -> bool {
        let s = num.to_string();
        let n = s.len();

        for i in 1..=n {
            let prefix: u32 = s[0..i].parse().unwrap();
            if !Self::is_prime(prefix) {
                return false;
            }
            let suffix: u32 = s[n - i..].parse().unwrap();
            if !Self::is_prime(suffix) {
                return false;
            }
        }
        true
    }

    fn is_prime(x: u32) -> bool {
        match x {
            0 | 1 => { return false; },
            2 | 3 => { return true; },
            _ => {},
        }
        if x % 2 == 0 || x % 3 == 0 {
            return false;
        }
        let mut i = 5;
        while i * i <= x {
            if x % i == 0 || x % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }
}