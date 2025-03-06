impl Solution {
    pub fn target_indices(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
        nums.sort();
        let mut res = Vec::new();
        for i in 0..nums.len() {
            if nums[i] == target {
                res.push(i as i32);
            }
        }
        res
    }
}