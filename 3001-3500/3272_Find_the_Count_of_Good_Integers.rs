impl Solution {
    pub fn count_good_integers(n: i32, k: i32) -> i64 {
        let nu = n as usize;
        let ku = k as usize;
        let base = 10i64.pow(((n - 1) / 2) as u32);
        let mut set = std::collections::HashSet::new();

        for i in base..base * 10 {
            let mut full = i;
            let mut mask = if n % 2 == 1 { 10 } else { 1 };
            while mask <= i {
                full = full * 10 + i / mask % 10;
                mask *= 10;
            }
            if full % k as i64 == 0 {
                let mut count = [0; 10];
                mask = 1;
                for _ in 0..n {
                    count[(full / mask % 10) as usize] += 1;
                    mask *= 10;
                }
                let mut key = 0i64;
                for d in 0..=9 {
                    key = key * 20 + count[d] as i64;
                }
                set.insert(key);
            }
        }

        let mut fact = vec![1i64; nu + 1];
        for i in 1..=nu {
            fact[i] = fact[i - 1] * i as i64;
        }

        let mut res = 0;
        for mut key in set {
            let mut count = [0; 10];
            for d in (0..=9).rev() {
                count[d] = key % 20;
                key /= 20;
            }
            let mut num = (n as i64 - count[0]) * fact[nu - 1];
            for d in 0..=9 {
                num /= fact[count[d] as usize];
            }
            res += num;
        }
        res
    }
}