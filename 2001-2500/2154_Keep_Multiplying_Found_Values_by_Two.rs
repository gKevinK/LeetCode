impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let mut s: std::collections::HashSet<_> = nums.iter().collect();
        let mut n = original;
        while s.contains(&n) {
            n *= 2;
        }
        n
    }
}