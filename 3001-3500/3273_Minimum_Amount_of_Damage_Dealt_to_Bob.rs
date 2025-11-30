impl Solution {
    pub fn min_damage(power: i32, damage: Vec<i32>, mut health: Vec<i32>) -> i64 {
        let n = damage.len();
        let mut ttime = 0;
        let mut tdmg = 0;
        for i in 0..n {
            health[i] = (health[i] + power - 1) / power;
            ttime += health[i];
            tdmg += damage[i];
        }
        let mut index = (0..n).map(|i| (damage[i] as f64 / health[i] as f64, i)).collect::<Vec<_>>();
        index.sort_unstable_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        let mut time = 0;
        let mut res = 0;
        for (_, i) in index {
            time += health[i];
            res += time as i64 * damage[i] as i64;
        }
        res
    }
}