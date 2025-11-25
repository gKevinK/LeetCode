impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let mut full = std::collections::HashMap::new();
        let mut dry = std::collections::BTreeSet::new();
        let mut result = vec![-1; rains.len()];

        for i in 0..rains.len() {
            if rains[i] == 0 {
                dry.insert(i);
                result[i] = 1;
                continue;
            }
            if let Some(full_day) = full.get(&rains[i]) {
                if let Some(&dry_day) = dry.range(full_day..).next() {
                    result[dry_day as usize] = rains[i];
                    dry.remove(&dry_day);
                } else {
                    return vec![];
                }
            }
            full.insert(rains[i], i);
        }
        result
    }
}