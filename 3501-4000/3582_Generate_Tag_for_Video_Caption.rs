impl Solution {
    pub fn generate_tag(caption: String) -> String {
        let mut res = String::from("#");
        let mut first = true;
        let mut cap = false;
        for c in caption.chars() {
            if c == ' ' {
                if !first {
                    cap = true;
                }
            } else {
                if cap {
                    res += &c.to_uppercase().to_string();
                    cap = false;
                } else {
                    res += &c.to_lowercase().to_string();
                }
                first = false;
            }
            if res.len() == 100 {
                break;
            }
        }
        res
    }
}