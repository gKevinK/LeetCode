impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut r = Vec::new();
        for i in 0..(nums.len() / 2) {
            let a = nums[i * 2];
            let b = nums[i * 2 + 1];
            for _ in 0..a {
                r.push(b);
            }
        }
        r
    }
}