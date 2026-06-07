impl Solution {
    pub fn max_gcd_score(nums: Vec<i32>, k: i32) -> i64 {
        use std::collections::VecDeque;
        let n = nums.len();
        let ku = k as usize;
        let mut segs = Vec::with_capacity(n);
        let mut pos = vec![VecDeque::with_capacity(ku + 1); 32];
        let mut res = 0;
        for i in 0..n {
            let x = nums[i] as i64;
            segs.push((0, i, i + 1));
            {
                let mut j = 0;
                let mut k = 0;
                while j < segs.len() {
                    let left = segs[j].1;
                    let g0 = Self::gcd(x, segs[j].0);
                    j += 1;
                    while j < segs.len() {
                        let g = Self::gcd(x, segs[j].0);
                        if g != g0 {
                            break;
                        }
                        j += 1;
                    }
                    segs[k] = (g0, left, segs[j - 1].2);
                    k += 1;
                }
                segs.truncate(k);
            }

            let mut q = &mut pos[x.trailing_zeros() as usize];
            if q.len() == ku + 1 {
                q.pop_front();
            }
            q.push_back(i);

            for &(g, l, r) in &segs {
                res = res.max((i + 1 - l) as i64 * g);

                let gz = g.trailing_zeros() as usize;
                let mut left = l;
                if pos[gz].len() == ku + 1 {
                    left = left.max(pos[gz][0] + 1);
                }
                res = res.max((i + 1 - left) as i64 * 2 * g);
            }
        }
        res
    }

    fn gcd(mut a: i64, mut b: i64) -> i64 {
        if a < b {
            (a, b) = (b, a);
        }
        while b != 0 {
            let t = a % b;
            a = b;
            b = t;
        }
        a
    }
}