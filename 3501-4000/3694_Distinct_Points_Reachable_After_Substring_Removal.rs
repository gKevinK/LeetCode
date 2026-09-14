impl Solution {
    pub fn distinct_points(s: String, k: i32) -> i32 {
        let bytes = s.as_bytes();
        let ku = k as usize;
        let mut set = std::collections::HashSet::with_capacity(bytes.len() - ku + 1);
        set.insert(0i64);
        let mut x = 0;
        let mut y = 0;
        for i in 0..(bytes.len() - ku) {
            match bytes[i + ku] {
                b'U' => y += 1,
                b'D' => y -= 1,
                b'L' => x -= 1,
                b'R' => x += 1,
                _ => {}
            };
            match bytes[i] {
                b'U' => y -= 1,
                b'D' => y += 1,
                b'L' => x += 1,
                b'R' => x -= 1,
                _ => {}
            }
            set.insert(x * 100000 + y);
        }
        set.len() as _
    }
}