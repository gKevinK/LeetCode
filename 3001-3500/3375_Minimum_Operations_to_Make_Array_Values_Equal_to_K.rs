impl Solution {
    pub fn min_operations(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        if nums[0] < k {
            return -1;
        }
        let mut res = 0;
        let mut last = *nums.last().unwrap();
        for i in (0..nums.len()).rev() {
            if nums[i] < last {
                res += 1;
                last = nums[i];
            }
        }
        res + i32::from(k < nums[0])
    }
}