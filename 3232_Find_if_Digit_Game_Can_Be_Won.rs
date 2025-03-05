impl Solution {
    pub fn can_alice_win(nums: Vec<i32>) -> bool {
        let mut sum1 = 0;
        let mut sum2 = 0;
        for n in nums {
            if n >= 10 {
                sum2 += n;
            } else {
                sum1 += n;
            }
        }
        sum1 != sum2
    }
}