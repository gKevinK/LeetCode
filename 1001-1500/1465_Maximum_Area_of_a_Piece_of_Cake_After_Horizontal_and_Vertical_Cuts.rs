impl Solution {
    pub fn max_area(h: i32, w: i32, mut horizontal_cuts: Vec<i32>, mut vertical_cuts: Vec<i32>) -> i32 {
        use std::cmp::max;
        let MOD = 1_000_000_007;
        horizontal_cuts.sort_unstable();
        vertical_cuts.sort_unstable();
        let mut ma = 0;
        let mut mb = 0;
        let mut last = 0;
        for i in horizontal_cuts {
            ma = max(ma, i - last);
            last = i;
        }
        ma = max(ma, h - last);
        last = 0;
        for i in vertical_cuts {
            mb = max(mb, i - last);
            last = i;
        }
        mb = max(mb, w - last);
        (ma as i64 * mb as i64 % MOD) as i32
    }
}