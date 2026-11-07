impl Solution {
    pub fn merge_adjacent(nums: Vec<i32>) -> Vec<i64> {
        let mut stack = Vec::new();
        for &num in &nums {
            let mut num = num as i64;
            while Some(&num) == stack.last() {
                stack.pop();
                num *= 2;
            }
            stack.push(num);
        }
        stack
    }
}