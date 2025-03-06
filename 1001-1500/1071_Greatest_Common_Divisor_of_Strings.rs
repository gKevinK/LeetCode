impl Solution {
    pub fn gcd_of_strings(mut str1: String, mut str2: String) -> String {
        if str1.len() < str2.len() {
            return Self::gcd_of_strings(str2, str1);
        }
        if str2.len() == 0 {
            return str1;
        }
        while str1.len() >= str2.len() {
            if str1[..str2.len()] != str2 {
                return "".to_string();
            }
            str1 = str1[str2.len()..].to_string();
        }
        Self::gcd_of_strings(str2, str1)
    }
}