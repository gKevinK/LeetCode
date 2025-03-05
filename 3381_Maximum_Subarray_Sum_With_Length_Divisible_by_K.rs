impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let ku = k as usize;
        let mut sums = vec![0i64; nums.len() + 1];
        for i in 0..nums.len() {
            sums[i + 1] = sums[i] + nums[i] as i64;
        }
        let mut res = sums[ku];
        for offset in 0..ku {
            let mut i = offset + ku;
            let mut min = 0i64;
            while i <= nums.len() {
                let s = sums[i] - sums[offset];
                res = res.max(s - min);
                min = min.min(s);
                i += ku;
            }
        }
        res
    }
}