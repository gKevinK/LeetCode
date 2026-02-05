impl Solution {
    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        match nums.len() {
            1 => 1,
            2 => 2,
            3 => 4,
            _ => {
                let mut m = 1;
                let mut l = nums.len();
                while l > 0 {
                    l >>= 1;
                    m <<= 1;
                }
                m
            }
        }
    }
}