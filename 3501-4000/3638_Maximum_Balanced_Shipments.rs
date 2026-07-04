impl Solution {
    pub fn max_balanced_shipments(weight: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut max = i32::MIN;
        for w in weight {
            if max <= w {
                max = w;
            } else {
                res += 1;
                max = i32::MIN;
            }
        }
        res
    }
}