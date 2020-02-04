impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        let mut v: Vec<_> = s.chars().collect();
        let len = v.len();
        if v.is_empty() {
            return 0;
        }
        let mut p = true;
        for i in 0..(len / 2) {
            if v[i] != v[len - i - 1] {
                p = false;
                break;
            }
        }
        if p { 1 } else { 2 }
    }
}