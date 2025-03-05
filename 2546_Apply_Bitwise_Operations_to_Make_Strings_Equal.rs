impl Solution {
    pub fn make_strings_equal(s: String, target: String) -> bool {
        let mut a1 = 0;
        let mut a0 = 0;
        for c in s.chars() {
            if c == '1' {
                a1 += 1;
            } else {
                a0 += 1;
            }
        }
        let mut b1 = 0;
        let mut b0 = 0;
        for c in target.chars() {
            if c == '1' {
                b1 += 1;
            } else {
                b0 += 1;
            }
        }
        if a1 == 0 && b1 > 0 {
            return false;
        } else if b1 == 0 && a1 > 0 {
            return false;
        }
        true
    }
}