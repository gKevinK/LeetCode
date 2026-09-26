impl Solution {
    pub fn max_sum_of_squares(num: i32, sum: i32) -> String {
        if sum > num * 9 {
            return String::from("");
        }
        let mut ans = String::new();
        for _ in 0..(sum / 9) {
            ans.push('9');
        }
        if sum % 9 > 0 {
            ans.push((b'0' + (sum % 9) as u8) as char);
        }
        while ans.len() < num as usize {
            ans.push('0');
        }
        ans
    }
}