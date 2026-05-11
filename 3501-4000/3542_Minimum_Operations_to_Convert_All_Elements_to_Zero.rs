impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut stack = vec![0];
        let mut res = 0;
        for n in nums {
            while n <= *stack.last().unwrap_or(&-1) {
                res += i32::from(n != *stack.last().unwrap_or(&-1));
                stack.pop();
            }
            stack.push(n);
        }
        res + stack.len() as i32 - 1
    }
}