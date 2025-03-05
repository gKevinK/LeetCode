impl Solution {
    pub fn count_days(days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        meetings.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
        let mut res = 0;
        let mut right = 0;
        for meeting in &meetings {
            if right < meeting[0] {
                res += meeting[0] - right - 1;
            }
            right = std::cmp::max(right, meeting[1]);
        }
        if days >= right {
            res += days - right;
        }
        res
    }
}