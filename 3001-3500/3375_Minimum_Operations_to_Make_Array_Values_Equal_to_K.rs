impl Solution {
    pub fn min_operations(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let mut res = 0;
        let mut last = *nums.last().unwrap();
        for i in (0..nums.len()).rev() {
            if nums[i] < k {
                return -1;
            }
            if nums[i] < last {
                res += 1;
                last = nums[i];
            }
        }
        res + if k < nums[0] { 1 } else { 0 }
    }
}