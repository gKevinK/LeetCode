impl Solution {
    pub fn maximum_score(nums: Vec<i32>, s: String) -> i64 {
        let mut heap = std::collections::BinaryHeap::new();
        let bytes = s.as_bytes();
        let mut score = 0;
        for i in 0..nums.len() {
            heap.push(nums[i]);
            if bytes[i] == b'1' {
                score += heap.pop().unwrap() as i64;
            }
        }
        score
    }
}