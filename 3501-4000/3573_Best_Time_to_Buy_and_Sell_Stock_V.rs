impl Solution {
    pub fn maximum_profit(prices: Vec<i32>, k: i32) -> i64 {
        let n = prices.len();
        let ku = k as usize;
        let mut b = vec![i64::MIN; n];
        let mut s = vec![0; n];
        let mut b2 = vec![i64::MIN; n];
        let mut s2 = vec![0; n];
        let mut res = 0;

        b[0] = -prices[0] as i64;
        s[0] = prices[0] as i64;
        for i in 1..n {
            b[i] = b[i - 1].max(-prices[i] as i64);
            s[i] = s[i - 1].max(prices[i] as i64);
        }
        for ik in 1..=ku {
            let mut p = 0;
            for i in (ik * 2 - 1)..n {
                p = p.max(prices[i] as i64 + b[i - 1]).max(-prices[i] as i64 + s[i - 1]);
                res = res.max(p);
                if i + 1 < n {
                    b2[i + 1] = b2[i].max(p - prices[i + 1] as i64);
                    s2[i + 1] = s2[i].max(p + prices[i + 1] as i64);
                }
            }
            std::mem::swap(&mut b, &mut b2);
            std::mem::swap(&mut s, &mut s2);
            if ik < ku {
                b2.fill(i64::MIN);
                s2.fill(0);
            }
        }
        res
    }
}