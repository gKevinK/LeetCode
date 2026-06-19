impl Solution {
    pub fn validate_coupons(code: Vec<String>, business_line: Vec<String>, is_active: Vec<bool>) -> Vec<String> {
        let mut v = vec![];
        for i in 0..code.len() {
            if is_active[i]
                && code[i].len() > 0
                && code[i].bytes().all(|b| (b'a' <= b && b <= b'z') || (b'A' <= b && b <= b'Z') || (b'0' <= b && b <= b'9') || b == b'_')
                && ["electronics", "grocery", "pharmacy", "restaurant"].iter().any(|&s| s == business_line[i]) {
                v.push((business_line[i].clone(), code[i].clone()));
            }
        }
        v.sort_unstable();
        v.iter().map(|x| x.1.clone()).collect()
    }
}