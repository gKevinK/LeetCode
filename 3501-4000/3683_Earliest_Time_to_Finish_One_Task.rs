impl Solution {
    pub fn earliest_time(tasks: Vec<Vec<i32>>) -> i32 {
        tasks.iter().map(|t| t[0] + t[1]).min().unwrap()
    }
}