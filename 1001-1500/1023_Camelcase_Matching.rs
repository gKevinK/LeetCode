impl Solution {
    fn m(q: & String, p: & String) -> bool {
        let mut j: usize = 0;
        for i in 0..q.len() {
            if q.chars().nth(i) == p.chars().nth(j) {
                j += 1;
            } else {
                if q.chars().nth(i).unwrap().is_uppercase() {
                    return false;
                }
            }
        }
        j == p.len()
    }
    pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        queries.into_iter().map(|q| Self::m(&q, &pattern)).collect()
    }
}