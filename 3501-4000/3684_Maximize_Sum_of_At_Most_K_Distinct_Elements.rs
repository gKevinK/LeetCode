impl Solution {
    pub fn max_k_distinct(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
        nums.sort_unstable_by_key(|&x| -x);
        nums.dedup();
        if nums.len() > k as usize {
            nums.drain(k as usize..);
        }
        nums
    }
}