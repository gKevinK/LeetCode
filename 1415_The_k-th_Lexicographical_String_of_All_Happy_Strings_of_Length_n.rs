impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let mut total = 3;
        for _ in 1..n {
            total *= 2;
        }
        if total < k {
            return String::from("");
        }
        let mut v = Vec::new();
        let mut k2 = k - 1;
        for _ in 1..n {
            v.push((k2 % 2) as u8);
            k2 /= 2;
        }
        let mut s = String::from("");
        let mut c = 'a' as u8 + k2 as u8;
        s.push(c as char);
        while let Some(mut i) = v.pop() {
            c = {
                let l = c - 'a' as u8;
                if l <= i {
                    i += 1;
                }
                'a' as u8 + i
            };
            s.push(c as char);
        }
        s
    }
}