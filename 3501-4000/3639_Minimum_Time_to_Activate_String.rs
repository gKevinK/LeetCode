impl Solution {
    pub fn min_time(s: String, order: Vec<i32>, k: i32) -> i32 {
        let n = s.len();
        let ns = n as i32;
        let mut count = ns as i64 * (ns as i64 + 1) / 2;
        if count < k as i64 {
            return -1;
        }
        let mut left  = (-1..ns - 1).collect::<Vec<_>>();
        let mut right = (1..=ns).collect::<Vec<_>>();

        for x in (0..n).rev() {
            let i = order[x];
            let l = left[i as usize];
            let r = right[i as usize];
            count -= (i - l) as i64 * (r - i) as i64;
            if count < k as i64 {
                return x as i32;
            }
            if l >= 0 {
                right[l as usize] = r;
            }
            if r < ns {
                left[r as usize] = l;
            }
        }
        -1
    }
}