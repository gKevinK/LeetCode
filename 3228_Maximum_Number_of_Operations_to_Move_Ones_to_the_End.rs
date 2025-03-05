impl Solution {
    pub fn max_operations(s: String) -> i32 {
        let s = s.as_bytes();
        let mut ops = 0;
        let mut ones = 0;
        let mut last = b'0';
        for &c in s {
            if c == b'1' {
                ones += 1;
            } else {
                if last == b'1' {
                    ops += ones;
                }
            }
            last = c;
        }
        ops
    }
}