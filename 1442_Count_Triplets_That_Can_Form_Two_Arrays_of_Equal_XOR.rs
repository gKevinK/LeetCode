impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut m = std::collections::HashMap::new();
        let mut r = 0;
        let mut x = 0;
        m.insert(0, vec![0]);
        for (i, n) in arr.iter().enumerate() {
            x ^= *n;
            let e = m.entry(x).or_insert(vec![]);
            for &l in e.iter() {
                r += i - l;
            }
            e.push(i + 1);
        }
        r as i32
    }
}