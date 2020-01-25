impl Solution {
    pub fn min_flips(mut a: i32, mut b: i32, mut c: i32) -> i32 {
        let mut r = 0;
        while a > 0 || b > 0 || c > 0 {
            let da = a & 1;
            let db = b & 1;
            let dc = c & 1;
            a >>= 1;
            b >>= 1;
            c >>= 1;
            if dc == 1 {
                r += if da == 0 && db == 0 { 1 } else { 0 };
            } else {
                r += da + db;
            }
        }
        r
    }
}