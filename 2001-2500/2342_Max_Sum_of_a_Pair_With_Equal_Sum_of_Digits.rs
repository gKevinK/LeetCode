impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut m = HashMap::new();
        let mut r = -1;
        for n in nums {
            let mut sum = 0;
            let mut cur = n;
            while cur > 0 {
                sum += cur % 10;
                cur /= 10;
            }
            match m.get(&sum) {
                Some(cand) => {
                    if *cand + n > r {
                        r = *cand + n;
                    };
                    if n > *cand {
                        m.insert(sum, n);
                    }
                },
                None => { m.insert(sum, n); }
            }
        }
        r
    }
}