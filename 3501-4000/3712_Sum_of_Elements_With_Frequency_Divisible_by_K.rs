impl Solution {
    pub fn sum_divisible_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = std::collections::HashMap::new();
        for &n in &nums {
            *map.entry(n).or_insert(0) += 1;
        }
        map.iter().map(|(n, v)| if v % k == 0 { n * v } else { 0 }).sum()
    }
}