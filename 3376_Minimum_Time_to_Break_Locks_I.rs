impl Solution {
    pub fn find_minimum_time(strength: Vec<i32>, k: i32) -> i32 {
        let n = strength.len();
        let mut v = vec![1_000_000_000; 1 << n];
        v[0] = 0;
        let mut factor = 1;
        for r in 0..n {
            let v1 = v.clone();
            for i in 0..n {
                let t = strength[i] / factor + if strength[i] % factor > 0 { 1 } else { 0 };
                for j in 0..(1 << n) {
                    if j & (1 << i) == 0 {
                        let k = j | (1 << i);
                        v[k] = v[k].min(v1[j] + t);
                    }
                }
            }
            factor += k;
        }
        *v.last().unwrap()
    }
}