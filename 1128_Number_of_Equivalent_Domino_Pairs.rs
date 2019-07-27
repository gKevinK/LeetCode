impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut map = std::collections::HashMap::new();
        for d in &dominoes {
            *map.entry(if d[0] < d[1] { (d[0], d[1]) } else { (d[1], d[0]) }).or_insert(0) += 1;
        }
        let mut res = 0;
        for (_, c) in & map {
            res += (c - 1) * c / 2;
        }
        res
    }
}