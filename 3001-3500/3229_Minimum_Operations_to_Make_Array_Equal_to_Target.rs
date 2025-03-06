impl Solution {
    pub fn minimum_operations(nums: Vec<i32>, mut target: Vec<i32>) -> i64 {
        for i in 0..nums.len() {
            target[i] = target[i] - nums[i];
        }
        let mut res = 0i64;
        let mut last = 0;
        for t in target {
            if t - last > 0 {
                res += t as i64 - last as i64;
            }
            last = t;
        }
        if last < 0 {
            res += -last as i64;
        }
        res
    }
}