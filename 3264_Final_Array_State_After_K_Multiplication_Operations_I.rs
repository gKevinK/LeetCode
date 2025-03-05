impl Solution {
    pub fn get_final_state(mut nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        for r in 0..k {
            let mut m_i = nums.len();
            let mut m_v = std::i32::MAX;
            for i in 0..nums.len() {
                if nums[i] < m_v {
                    m_i = i;
                    m_v = nums[i];
                }
            }
            nums[m_i] = nums[m_i] * multiplier;
        }
        nums
    }
}