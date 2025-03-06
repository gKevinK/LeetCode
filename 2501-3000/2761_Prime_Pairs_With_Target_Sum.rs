impl Solution {
    pub fn find_prime_pairs(n: i32) -> Vec<Vec<i32>> {
        if n < 4 {
            return vec![];
        }
        let un = n as usize;
        let mut prime = vec![true; un + 1];
        prime[0] = false;
        prime[1] = false;
        for i in 2..(un / 2) {
            if i * i > un {
                break;
            }
            if prime[i] {
                let mut j = i * i;
                while j < un + 1 {
                    prime[j] = false;
                    j += i;
                }
            }
        }
        let mut res = vec![];
        for i in 2..=(un / 2) {
            if prime[i] && prime[un - i] {
                res.push(vec![i as i32, n - i as i32]);
            }
        }
        res
    }
}