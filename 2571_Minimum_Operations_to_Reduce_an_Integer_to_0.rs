impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        use std::cmp::min;
        let mut n0 = 0;
        let mut n1 = 1;
        let mut t = n;
        while t > 0 {
            if t % 2 == 0 {
                let tn0 = min(n0, n1 + 1);
                let tn1 = min(n0 + 2, n1 + 1);
                n0 = tn0;
                n1 = tn1;
            } else {
                let tn0 = min(n0 + 1, n1 + 2);
                let tn1 = min(n0 + 1, n1);
                n0 = tn0;
                n1 = tn1;
            }
            t /= 2;
        }
        min(n0, n1 + 1) 
    }
}