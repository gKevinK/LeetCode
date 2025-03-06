impl Solution {
    pub fn cells_in_range(s: String) -> Vec<String> {
        let v = s.as_bytes();
        let mut res = Vec::new();
        for i in v[0]..=v[3] {
            for j in v[1]..=v[4] {
                res.push(String::from_utf8(vec![i, j]).unwrap());
            }
        }
        res
    }
}