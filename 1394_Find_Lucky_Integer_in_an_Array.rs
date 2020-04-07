impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut m = std::collections::HashMap::new();
        for i in &arr {
            *m.entry(i).or_insert(0) += 1;
        }
        let mut r = -1;
        for (k, v) in &m {
            if *k == v && *v > r {
                r = *v;
            }
        }
        r
    }
}