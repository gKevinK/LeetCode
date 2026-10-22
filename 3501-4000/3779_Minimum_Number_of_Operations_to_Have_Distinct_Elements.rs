impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut rm = 0;
        let mut set = [false; 100_001];
        for i in (0..n).rev() {
            let idx = nums[i] as usize;
            if set[idx] {
                rm = i + 1;
                break;
            }
            set[idx] = true;
        }
        ((rm + 2) / 3) as i32
    }
}