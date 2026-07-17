impl Solution {
    pub fn min_operations(s: String, k: i32) -> i32 {
        let k = k as usize;
        let n = s.len();
        let mut z = s.bytes().filter(|&b| b == b'0').count();

        if z == 0 {
            return 0;
        }
        if n == k {
            return if z == n { 1 } else { -1 };
        }

        let mut res = usize::MAX;
        if z % 2 == 0 {
            let mut c = z.div_ceil(k).max(z.div_ceil(n - k));
            c += c % 2;
            res = res.min(c);
        }
        if z % 2 == k % 2 {
            let mut c = z.div_ceil(k).max((n - z).div_ceil(n - k));
            c += 1 - c % 2;
            res = res.min(c);
        }
        if res < usize::MAX { res as i32 } else { -1 }
    }
}