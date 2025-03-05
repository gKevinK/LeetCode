impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let mut num = 0i64;
        let mut start = 0;
        for i in 0..prices.len() {
            if start < i && prices[i - 1] - 1 == prices[i] {
                num += (i - start + 1) as i64;
            } else {
                num += 1;
                start = i;
            }
        }
        num
    }
}