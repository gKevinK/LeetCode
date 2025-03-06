impl Solution {
    pub fn min_difference(mut nums: Vec<i32>) -> i32 {
        if nums.len() <= 4 {
            return 0;
        }
        nums.sort();
        let mut diff = nums.last().unwrap() - nums.first().unwrap();
        let mut l = 0;
        let mut r = nums.len() - 4;
        while r < nums.len() {
            diff = std::cmp::min(diff, nums[r] - nums[l]);
            l += 1;
            r += 1;
        }
        diff
    }
}