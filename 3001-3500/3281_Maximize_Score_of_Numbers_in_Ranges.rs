impl Solution {
    pub fn max_possible_score(mut start: Vec<i32>, d: i32) -> i32 {
        start.sort_unstable();
        let len = start.len();
        let mut lo = 0;
        let mut hi = start[len - 1] - start[0] + d;
        while lo < hi {
            let mi = lo + (hi - lo + 1) / 2;
            if Self::check(&start, d, mi) {
                lo = mi;
            } else {
                hi = mi - 1;
            }
        }
        lo
    }

    fn check(start: &Vec<i32>, d: i32, m: i32) -> bool {
        let mut prev = start[0] as i64;
        for i in 1..start.len() {
            let this = (prev + m as i64).max(start[i] as i64);
            if this > (start[i] + d) as i64 {
                return false;
            }
            prev = this;
        }
        true
    }
}