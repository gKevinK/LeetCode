impl Solution {
    pub fn sort_permutation(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut mask = (1 << 30) - 1;
        for i in 0..n {
            if nums[i] != i as i32 {
                mask &= nums[i];
            }
        }
        if mask == (1 << 30) - 1 {
            return 0;
        }
        mask
    }
}