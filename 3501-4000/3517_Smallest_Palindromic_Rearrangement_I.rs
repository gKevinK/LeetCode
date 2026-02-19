impl Solution {
    pub fn smallest_palindrome(s: String) -> String {
        let mut count = vec![0; 26];
        for b in s.bytes() {
            count[(b - b'a') as usize] += 1;
        }
        let mut mid = 0;
        let mut res = vec![];
        for i in 0..26 {
            if count[i as usize] % 2 == 1 {
                mid = i + b'a';
            }
            for _ in 0..(count[i as usize] / 2) {
                res.push(i + b'a');
            }
        }
        let rev = res.clone();
        if mid > 0 {
            res.push(mid);
        }
        res.extend(rev.iter().rev());
        String::from_utf8(res).unwrap()
    }
}