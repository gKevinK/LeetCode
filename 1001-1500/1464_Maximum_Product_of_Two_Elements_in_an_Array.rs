impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut m1 = 0;
        let mut m2 = 0;
        for n in nums {
            if n > m1 {
                m2 = m1;
                m1 = n;
            } else if n > m2 {
                m2 = n;
            }
        }
        (m1 - 1) * (m2 - 1)
    }
}