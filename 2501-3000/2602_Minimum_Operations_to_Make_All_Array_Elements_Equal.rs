impl Solution {
    pub fn min_operations(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i64> {
        let n = nums.len();
        nums.sort_unstable();
        let mut v = vec![0; n];
        for i in 1..n {
            v[i] = v[i - 1] + (nums[i] - nums[i - 1]) as i64 * i as i64;
        }
        let sum = nums[n - 1] as i64 * n as i64 - v[n - 1];

        queries.iter().map(|&q| {
            let pp = nums.partition_point(|&x| x < q);
            let x = if pp > 0 { v[pp - 1] + pp as i64 * (q - nums[pp - 1]) as i64 } else { 0 };
            sum - n as i64 * q as i64 + x * 2
        }).collect()
    }
}