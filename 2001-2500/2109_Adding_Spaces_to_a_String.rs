impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut r = String::from("");
        let mut i = 0;
        for is in spaces {
            r.push_str(&s[i..is as usize]);
            r.push(' ');
            i = is as usize;
        }
        r.push_str(&s[i..(s.len())]);
        r
    }
}