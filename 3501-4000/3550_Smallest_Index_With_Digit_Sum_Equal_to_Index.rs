impl Solution {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            let mut n = nums[i];
            let mut s = 0;
            while n > 0 {
                s += n % 10;
                n /= 10;
            }
            if i as i32 == s {
                return i as i32;
            }
        }
        -1
    }
}