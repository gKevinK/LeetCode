impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        let mut v: Vec<_> = palindrome.chars().collect();
        let l = v.len();
        if l == 1 {
            return String::from("");
        }
        let mut r = false;
        for i in 0..(l / 2) {
            if v[i] != 'a' {
                v[i] = 'a';
                r = true;
                break;
            }
        }
        if !r {
            v[l - 1] = 'b';
        }
        v.into_iter().collect()
    }
}