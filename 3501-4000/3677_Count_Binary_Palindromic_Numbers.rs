impl Solution {
    pub fn count_binary_palindromes(n: i64) -> i32 {
        let mut lo = 0;
        let mut hi = 1 << 30;
        while lo < hi {
            let mi = lo + (hi - lo) / 2;
            if Self::full(mi, false) > n {
                hi = mi;
            } else {
                lo = mi + 1;
            }
        }
        let n_even = lo - 1;

        lo = 0;
        hi = 1 << 30;
        while lo < hi {
            let mi = lo + (hi - lo) / 2;
            if Self::full(mi, true) > n {
                hi = mi;
            } else {
                lo = mi + 1;
            }
        }
        let n_odd = lo;

        (n_even + n_odd) as _
    }

    fn full(x: i64, mid: bool) -> i64 {
        let mut res = x;
        let mut mask = 1;
        if mid {
            mask <<= 1;
        }
        while mask <= res {
            res = (res << 1) + i64::from(res & mask > 0);
            mask <<= 2;
        }
        res
    }
}