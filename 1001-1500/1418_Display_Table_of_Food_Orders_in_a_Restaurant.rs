impl Solution {
    pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut tables = std::collections::BTreeSet::new();
        let mut foods = std::collections::BTreeSet::new();
        let mut m = std::collections::HashMap::new();
        for o in &orders {
            tables.insert(o[1].parse::<i32>().unwrap());
            foods.insert(&o[2]);
            *m.entry((&o[1], &o[2])).or_insert(0) += 1;
        }
        let mut r = vec![vec![String::from("Table")]];
        for f in foods {
            r[0].push(f.to_string());
        }
        for t in tables {
            let t_str = t.to_string();
            let mut v = vec![t_str.clone()];
            for i in 1..r[0].len() {
                v.push(m.get(&(&t_str, &r[0][i])).unwrap_or(&0).to_string());
            }
            r.push(v);
        }
        r
    }
}