impl Solution {
    pub fn min_cost(nums: Vec<i32>, k: i32) -> i32 {
        let len = nums.len();
        let mut min_costs = vec![2_000_000_000; len + 1];
        min_costs[0] = 0;
        for left in 0..len {
            let lcost = min_costs[left];
            let mut counts = vec![0; len];
            let mut ivalue = k;
            for right in left..len {
                ivalue += match counts[nums[right] as usize] {
                    0 => 0,
                    1 => 2,
                    _ => 1,
                };
                counts[nums[right] as usize] += 1;
                min_costs[right + 1] = std::cmp::min(min_costs[right + 1], lcost + ivalue);
            }
        }
        min_costs[len]
    }
}