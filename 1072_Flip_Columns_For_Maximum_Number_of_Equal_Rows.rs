impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let mut map: HashMap<String, i32> = HashMap::new();
        for row in &matrix {
            let mut s = "".to_string();
            for ch in row {
                s.push(if *ch == row[0] { '0' } else { '1' });
            }
            *map.entry(s).or_insert(0) += 1;
        }
        let mut r = 1;
        for v in map.values() {
            r = std::cmp::max(r, *v);
        }
        r
    }
}