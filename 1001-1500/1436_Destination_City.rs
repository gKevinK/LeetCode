impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut m = std::collections::HashMap::new();
        for p in paths {
            m.insert(p[0].clone(), true);
            m.entry(p[1].clone()).or_insert(false);
        }
        for (k, v) in m {
            if v == false {
                return k;
            }
        }
        String::from("")
    }
}