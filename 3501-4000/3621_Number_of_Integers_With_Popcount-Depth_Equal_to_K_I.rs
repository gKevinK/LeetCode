impl Solution {
    pub fn popcount_depth(n: i64, k: i32) -> i64 {
        if k == 0 {
            return 1;
        }
        let mut pd = vec![0; 51];
        for i in 1..51 {
            let mut p: usize = i;
            let mut depth = 0;
            while p > 1 {
                p = p.count_ones() as _;
                depth += 1;
            }
            pd[i] = depth;
        }

        let mut res = 0;
        for i in 1..51 {
            if pd[i] == k - 1 {
                let mut rest = i as i64;
                for bit in (0..=51).rev() {
                    if (1 << bit) & n > 0 {
                        if bit >= rest {
                            res += Self::comb(bit, rest);
                        }
                        rest -= 1;
                    }
                }
                res += (rest == 0) as i64;
                if i == 1 {
                    res -= 1;
                }
            }
        }
        res
    }

    fn comb(n: i64, mut m: i64) -> i64 {
        if m > n {
            return 0;
        }
        let mut res = 1;
        for i in (m + 1)..=n {
            res *= i;
            res /= (i - m);
        }
        res
    }
}