impl Solution {
    pub fn max_balanced_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut map = std::collections::HashMap::new();
        map.insert((0, 0), 0);
        let mut res = 0;
        let mut diff = 0;
        let mut xor = 0;
        for i in 0..n {
            xor ^= nums[i];
            if nums[i] % 2 == 0 {
                diff += 1;
            } else {
                diff -= 1;
            }
            match map.entry((xor, diff)) {
                std::collections::hash_map::Entry::Occupied(e) => {
                    res = res.max(i as i32 + 1 - *e.get());
                },
                std::collections::hash_map::Entry::Vacant(e) => {
                    e.insert(i as i32 + 1);
                }
            }
        }
        res
    }
}