impl Solution {
    pub fn max_subarrays(n: i32, conflicting_pairs: Vec<Vec<i32>>) -> i64 {
        let mut v = vec![(0, 0); n as usize + 1];
        for pair in conflicting_pairs {
            let a = std::cmp::min(pair[0], pair[1]);
            let b = std::cmp::max(pair[0], pair[1]);
            let left = &mut v[b as usize];
            if a >= left.0 {
                left.1 = left.0;
                left.0 = a;
            } else if a > left.1 {
                left.1 = a;
            }
        }
        let mut res = 0i64;
        let mut gain = vec![0i64; n as usize + 1];
        let mut left = (0, 0);
        for i in 1..=n {
            for a in [v[i as usize].0, v[i as usize].1] {
                if a >= left.0 {
                    left.1 = left.0;
                    left.0 = a;
                } else if a > left.1 {
                    left.1 = a;
                }
            }
            res += (i - left.0) as i64;
            gain[left.0 as usize] += (left.0 - left.1) as i64;
        }
        res + gain.iter().max().unwrap()
    }
}