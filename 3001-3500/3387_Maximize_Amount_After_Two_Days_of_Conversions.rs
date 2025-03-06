impl Solution {
    pub fn max_amount(initial_currency: String, pairs1: Vec<Vec<String>>, rates1: Vec<f64>, pairs2: Vec<Vec<String>>, rates2: Vec<f64>) -> f64 {
        use std::collections::HashMap;

        let mut m1 = HashMap::new();
        m1.insert(initial_currency.clone(), 1.0f64);
        for _ in 0..pairs1.len() {
            let mut f = false;
            for i in 0..pairs1.len() {
                if let Some(&c) = m1.get(&pairs1[i][0]) {
                    let e = m1.entry(pairs1[i][1].clone()).or_insert(0.0f64);
                    *e = f64::max(*e, c * rates1[i]);
                    f = true;
                }
                if let Some(&c) = m1.get(&pairs1[i][1]) {
                    let e = m1.entry(pairs1[i][0].clone()).or_insert(0.0f64);
                    *e = f64::max(*e, c / rates1[i]);
                    f = true;
                }
            }
            if f == false {
                break;
            }
        }
        let mut res = 1.0f64;
        for _ in 0..pairs2.len() {
            for i in 0..pairs2.len() {
                if let Some(&c) = m1.get(&pairs2[i][0]) {
                    let e = m1.entry(pairs2[i][1].clone()).or_insert(0.0f64);
                    *e = f64::max(*e, c * rates2[i]);
                }
                if let Some(&c) = m1.get(&pairs2[i][1]) {
                    let e = m1.entry(pairs2[i][0].clone()).or_insert(0.0f64);
                    *e = f64::max(*e, c / rates2[i]);
                }
            }
        }
        *m1.get(&initial_currency).unwrap_or(&1.0f64)
    }
}