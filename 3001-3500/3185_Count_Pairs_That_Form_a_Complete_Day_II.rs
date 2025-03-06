impl Solution {
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i64 {
        let mut r = 0;
        let mut v = [0; 24];
        for i in 0..hours.len() {
            let h = hours[i] as u32 % 24;
            let c = (24 - h) % 24;
            r += v[c as usize];
            v[h as usize] += 1;
        }
        r
    }
}