impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        let mut curr = 0;
        let mut res = Vec::new();
        for c in seq.chars() {
            if c == '(' {
                res.push(curr % 2);
                curr += 1;
            } else {
                curr -= 1;
                res.push(curr % 2);
            };
        }
        res
    }
}