impl Solution {
    pub fn non_special_count(l: i32, r: i32) -> i32 {
        let mut prime = vec![true; (r as f64).sqrt() as usize + 1];
        prime[0] = false;
        let mut count = 0;
        for i in 2..prime.len() {
            if prime[i] {
                let mut j = i * 2;
                while j < prime.len() {
                    prime[j] = false;
                    j += i;
                }
                let pow = (i as i32).pow(2);
                if l <= pow && pow <= r {
                    count += 1;
                }
            }
        }
        r - l + 1 - count
    }
}