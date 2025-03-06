impl Solution {
    pub fn string_hash(s: String, k: i32) -> String {
        let vs = s.as_bytes();
        let l = s.len() / k as usize;
        let mut vr = vec![0u8; l];
        for i in 0..l {
            let mut sum = 0i32;
            for j in (i * k as usize)..((i + 1) * k as usize) {
                sum += (vs[j] - 'a' as u8) as i32;
            }
            sum %= 26;
            vr[i] = sum as u8 + 'a' as u8;
        }
        String::from_utf8(vr).unwrap()
    }
}