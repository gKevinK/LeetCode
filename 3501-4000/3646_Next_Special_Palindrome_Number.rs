impl Solution {
    pub fn special_palindrome(n: i64) -> i64 {
        use std::sync::LazyLock;
        static P: LazyLock<Vec<i64>> = LazyLock::new(|| {
            let mut p = vec![];
            for mask in 1..(1i64 << 9) {
                if (mask & 0b101010101).count_ones() > 1 {
                    continue;
                }
                let mut len = 0;
                let mut rest = 0;
                let mut m = 1;
                for d in 1..=9 {
                    if (1 << (d - 1)) & mask > 0 {
                        len += d;
                        rest += d * m;
                    }
                    m *= 10;
                }
                if len > 16 {
                    continue;
                }
                Solution::dfs(&mut p, rest, 0);
            }
            p.sort_unstable();
            p.dedup();
            p
        });

        let i = P.partition_point(|&x| x <= n);
        P[i]
    }

    fn dfs(mut v: &mut Vec<i64>, m: i64, r: i64) {
        if m == 0 {
            v.push(Self::full(r, false));
            return;
        }
        let mut x = 100_000_000;
        let mut d = 9;
        while x > 0 {
            let dn = m / x % 10;
            if dn >= 2 {
                Self::dfs(&mut v, m - x * 2, r * 10 + d);
            } else if m - x == 0 {
                v.push(Self::full(r * 10 + d, true));
                break;
            }
            x /= 10;
            d -= 1;
        }
    }

    fn full(x: i64, mid: bool) -> i64 {
        let mut res = x;
        let mut mask = 1;
        if mid {
            mask *= 10;
        }
        while mask <= res {
            res = (res * 10) + res / mask % 10;
            mask *= 100;
        }
        res
    }
}