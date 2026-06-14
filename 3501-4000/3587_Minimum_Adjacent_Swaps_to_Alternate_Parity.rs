impl Solution {
    pub fn min_swaps(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut r1 = 0;
        for i in 0..n {
            nums[i] &= 1;
            r1 += nums[i];
        }
        let mut r0 = n as i32 - r1;
        if r0.abs_diff(r1) > 1 {
            return -1;
        }
        let mut res = i32::MAX;
        if r0 >= r1 {
            let mut total = 0;
            let mut need1 = 0;
            let mut has1 = 0;
            for i in 0..n {
                need1 += (i & 1) as i32;
                has1 += nums[i];
                total += (need1 - has1).abs();
            }
            res = res.min(total);
        }
        if r0 <= r1 {
            let mut total = 0;
            let mut need1 = 0;
            let mut has1 = 0;
            for i in 0..n {
                need1 += 1 - (i & 1) as i32;
                has1 += nums[i];
                total += (need1 - has1).abs();
            }
            res = res.min(total);
        }
        res
    }
}