impl Solution {
    pub fn find_prime_pairs(n: i32) -> Vec<Vec<i32>> {
        if n < 4 {
            return vec![];
        }
        let un = n as usize;
        use std::sync::OnceLock;
        static PRIME: OnceLock<Vec<bool>> = OnceLock::new();
        let prime = PRIME.get_or_init(|| {
            let mut p = vec![true; 1000001];
            p[0] = false;
            p[1] = false;
            for i in 2..=(500000) {
                if i * i >= p.len() {
                    break;
                }
                if p[i] {
                    let mut j = i * i;
                    while j < p.len() {
                        p[j] = false;
                        j += i;
                    }
                }
            }
            p
        });
        let mut res = vec![];
        for i in 2..=(un / 2) {
            if prime[i] && prime[un - i] {
                res.push(vec![i as i32, n - i as i32]);
            }
        }
        res
    }
}