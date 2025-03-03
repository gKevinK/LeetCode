impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::HashMap;
        
        let mut m = HashMap::new();
        let mut sum = 0;
        m.insert(0, -1_i32);
        for (i, x) in nums.iter().enumerate() {
            sum += x;
            if k != 0 {
                sum %= k;
            }
            let t = match m.get(&sum) {
                Some(r) => r + 1 < i as i32,
                _ => {
                    m.insert(sum, i as i32);
                    false
                }
            };
            if t {
                return true;
            }
        }
        false
    }
}