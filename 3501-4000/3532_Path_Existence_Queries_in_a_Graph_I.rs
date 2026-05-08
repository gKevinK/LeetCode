impl Solution {
    pub fn path_existence_queries(n: i32, nums: Vec<i32>, max_diff: i32, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut g = vec![0; n as usize];
        let mut i = 0;
        while i < nums.len() {
            g[i] = i;
            let mut j = i + 1;
            while j < nums.len() && (nums[j] - nums[j - 1]).abs() <= max_diff {
                g[j] = i;
                j += 1;
            }
            i = j;
        }
        queries.iter().map(|q| g[q[0] as usize] == g[q[1] as usize]).collect()
    }
}