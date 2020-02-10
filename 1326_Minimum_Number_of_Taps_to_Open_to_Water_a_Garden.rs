impl Solution {
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let mut v = Vec::new();
        for i in 0..=n {
            v.push((i - ranges[i as usize], i + ranges[i as usize]));
        }
        v.sort();
        let mut start = 0;
        let mut i = 0;
        let mut result = 0;
        while start < n {
            if i >= v.len() || start < v[i].0 {
                return -1;
            }
            let mut reach = 0;
            while i < v.len() && v[i].0 <= start {
                reach = std::cmp::max(reach, v[i].1);
                i += 1;
            }
            start = reach;
            result += 1;
        }
        result
    }
}