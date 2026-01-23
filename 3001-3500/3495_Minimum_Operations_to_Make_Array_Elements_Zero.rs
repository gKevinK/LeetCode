impl Solution {
    pub fn min_operations(queries: Vec<Vec<i32>>) -> i64 {
        let mut powers = vec![1i64];
        while powers[powers.len() - 1] < 1_000_000_000 {
            powers.push(powers[powers.len() - 1] * 4);
        }
        let mut result = 0i64;
        for query in queries {
            let r = query[1] as i64;
            let mut need = 0i64;
            let mut curr = query[0] as i64;
            let mut i = 1;
            while curr > powers[i] {
                i += 1;
            }
            loop {
                if r < powers[i] {
                    need += (r - curr + 1) * i as i64;
                    break;
                }
                need += (powers[i] - curr) * i as i64;
                curr = powers[i];
                i += 1;
            }
            result += (need + 1) / 2;
        }
        result
    }
}