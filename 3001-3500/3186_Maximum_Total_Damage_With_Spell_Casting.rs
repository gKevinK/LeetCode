impl Solution {
    pub fn maximum_total_damage(mut power: Vec<i32>) -> i64 {
        power.sort_unstable();
        let mut c = 0;
        let mut total_c = 0;
        let mut total_m1 = 0;
        let mut total_m2 = 0;
        for p in power {
            let p = p as i64;
            let mut max = total_c;
            if c == p {
                // pass
            } else if c + 1 == p {
                max = total_m2;
                total_m2 = total_m2.max(total_m1);
                total_m1 = total_c;
            } else if c + 2 == p {
                max = total_m2.max(total_m1);
                total_m2 = max.max(total_c);
                total_m1 = total_c;
            } else {
                max = total_m2.max(total_m1).max(total_c);
                total_m2 = max;
                total_m1 = max;
            }
            total_c = max + p;
            c = p;
        }
        total_c.max(total_m1).max(total_m2)
    }
}