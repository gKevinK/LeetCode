impl Solution {
    pub fn concat_hex36(n: i32) -> String {
        let mut s = String::new();
        let mut n1 = n * n * n;
        let mut d = vec![];
        while n1 > 0 {
            d.push(n1 % 36);
            n1 /= 36;
        }
        n1 = n * n;
        while n1 > 0 {
            d.push(n1 % 16);
            n1 /= 16;
        }
        for &i in d.iter().rev() {
            if i < 10 {
                s.push((i as u8 + b'0') as char);
            } else {
                s.push((i as u8 - 10 + b'A') as char);
            }
        }
        s
    }
}