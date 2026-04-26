impl Solution {
    pub fn maximum_possible_size(nums: Vec<i32>) -> i32 {
        let mut curr = 0;
        let mut size = 0;
        for n in nums {
            if n >= curr {
                size += 1;
                curr = n;
            }
        }
        size
    }
}