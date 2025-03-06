impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut set = std::collections::HashSet::new();
        set.insert(0);
        for s in stones {
            sum += s;
            for a in set.iter().cloned().collect::<Vec<i32>>() {
                set.insert(s + a);
            }
        }
        let mut r = sum;
        for a in set {
            r = std::cmp::min(r, (sum - a - a).abs());
        }
        r
    }
}