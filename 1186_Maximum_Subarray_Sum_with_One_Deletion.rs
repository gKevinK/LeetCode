impl Solution {
    pub fn maximum_sum(arr: Vec<i32>) -> i32 {
        use std::cmp::max;
        
        let mut a = -1_000_000;
        let mut b = -1_000_000;
        let mut r = std::i32::MIN;
        for &i in &arr {
            b = max(a, b + i);
            a = max(a + i, i);
            r = max(r, max(a, b));
        }
        r
    }
}