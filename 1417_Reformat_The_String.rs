impl Solution {
    pub fn reformat(s: String) -> String {
        let v: Vec<_> = s.chars().collect();
        let mut r = String::from("");
        let num_alpha = v.iter().filter(|x| x.is_ascii_lowercase()).count();
        let num_digit = v.iter().filter(|x| x.is_ascii_digit()).count();
        if (num_alpha as i32 - num_digit as i32).abs() > 1 {
            return r;
        }
        let mut i1 = v.iter().filter(|x| x.is_ascii_lowercase());
        let mut i2 = v.iter().filter(|x| x.is_ascii_digit());
        let mut n = if num_digit > num_alpha { 1 } else { 0 };
        loop {
            if n == 0 {
                match i1.next() {
                    Some(&c) => { r.push(c) },
                    None => break,
                }
            } else {
                match i2.next() {
                    Some(&c) => { r.push(c) },
                    None => break,
                }
            }
            n = (n + 1) % 2;
        }
        r
    }
}