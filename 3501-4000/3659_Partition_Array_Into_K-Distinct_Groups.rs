impl Solution {
    pub fn partition_array(nums: Vec<i32>, k: i32) -> bool {
        if nums.len() % (k as usize) != 0 {
            return false;
        }
        let g = nums.len() as i32 / k;
        let mut m = [0; 100_001];
        for num in nums {
            m[num as usize] += 1;
            if m[num as usize] > g {
                return false;
            }
        }
        true
    }
}