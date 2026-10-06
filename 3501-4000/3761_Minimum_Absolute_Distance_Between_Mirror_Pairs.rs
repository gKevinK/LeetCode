impl Solution {
    pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
        let mut res = usize::MAX;
        let mut map = std::collections::HashMap::new();
        for i in 0..nums.len() {
            let num = nums[i] as i64;
            let mut tmp = num;
            let mut rev = 0;
            while tmp > 0 {
                rev = rev * 10 + tmp % 10;
                tmp /= 10;
            }
            if let Some(j) = map.get(&num) {
                res = res.min(i - j);
            }
            map.insert(rev, i);
        }
        if res == usize::MAX { -1 } else { res as i32 }
    }
}