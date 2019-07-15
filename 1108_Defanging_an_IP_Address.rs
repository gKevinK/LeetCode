impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        let mut res = String::new();
        for c in address.chars() {
            if c == '.' {
                res += "[.]";
            } else {
                res.push(c);
            }
        }
        res
    }
}