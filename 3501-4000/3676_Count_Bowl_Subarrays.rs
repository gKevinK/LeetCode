impl Solution {
    pub fn bowl_subarrays(nums: Vec<i32>) -> i64 {
        let mut res = 0;
        let n = nums.len();
        let mut stack = Vec::with_capacity(n);
        for num in nums {
            while *stack.last().unwrap_or(&i32::MAX) <= num {
                let m = stack.pop().unwrap();
                res += (m < num && !stack.is_empty()) as i64;
            }
            stack.push(num);
        }
        res
    }
}