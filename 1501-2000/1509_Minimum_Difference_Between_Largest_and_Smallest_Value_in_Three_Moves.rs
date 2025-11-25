impl Solution {
    pub fn min_difference(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 4 {
            return 0;
        }
        if n <= 7 {
            nums.sort();
        } else {
            let (l, _, r) = nums.select_nth_unstable(n - 4);
            r.sort();
            let (l, _, _) = l.select_nth_unstable(3);
            l.sort();
        }
        let mut diff = i32::MAX;
        for i in 0..=3 {
            diff = diff.min(nums[n - 4 + i] - nums[i]);
        }
        diff
    }
}