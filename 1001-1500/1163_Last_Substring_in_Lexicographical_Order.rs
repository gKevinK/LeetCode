impl Solution {
    pub fn last_substring(s: String) -> String {
        let str: Vec<char> = s.chars().collect();
        let n = str.len();
        let mut i = 0;
        let mut j = 1;
        let mut k = 0;
        while j + k < n {
            if str[i + k] == str[j + k] {
                k += 1;
            } else if str[i + k] < str[j + k] {
                i = std::cmp::max(i + k + 1, j);
                j = i + 1;
                k = 0;
            } else {
                j = j + k + 1;
                k = 0;
            }
        }
        s[i..].to_string()
    }
}