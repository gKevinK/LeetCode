impl Solution {
    pub fn abs_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        let mut suml = 0;
        let mut sums = 0;
        for i in 0..k as usize {
            sums += nums[i];
            suml += nums[n - i - 1];
        }
        suml - sums
    }
}