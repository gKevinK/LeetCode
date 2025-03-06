impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        let mut diff = 0;
        for mut n in nums {
            n /= 10;
            let mut m = 9;
            while n > 0 {
                diff += (n % 10) * m;
                n /= 10;
                m = m * 10 + 9;
            }
        }
        diff
    }
}