impl Solution {
    pub fn min_moves(balance: Vec<i32>) -> i64 {
        let n = balance.len();
        let mut i = 0;
        while i < n && balance[i] >= 0 {
            i += 1;
        }
        if i == n {
            return 0;
        }
        let mut need = -balance[i] as i64;
        let mut d = 1;
        let mut res = 0;
        while need > 0 && d * 2 <= n {
            let l = if d > i { balance[i + n - d] } else { balance[i - d] } as i64;
            let r = balance[(i + d) % n] as i64;
            if need > 0 {
                let mv = need.min(l);
                res += mv * d as i64;
                need -= mv;
            }
            if need > 0 && d * 2 < n {
                let mv = need.min(r);
                res += mv * d as i64;
                need -= mv;
            }
            if need <= 0 {
                return res;
            }
            d += 1;
        }
        -1
    }
}