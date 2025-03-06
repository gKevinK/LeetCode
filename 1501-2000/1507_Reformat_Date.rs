impl Solution {
    pub fn reformat_date(date: String) -> String {
        let v: Vec<_> = date.split(' ').collect();
        let mut day = 0;
        for c in v[0].chars() {
            if !c.is_digit(10) {
                break;
            }
            day = day * 10 + (c as i32 - '0' as i32);
        }
        let month = vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"].iter().position(|&s| s == v[1]).unwrap_or(12) as i32 + 1;
        String::from(v[2]) + "-" + &format!("{:02}", month) + "-" + &format!("{:02}", day)
    }
}