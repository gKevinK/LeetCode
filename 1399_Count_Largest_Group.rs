impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut v = vec![0; 40];
        for mut i in 1..=n {
            let mut g = 0;
            while i > 0 {
                g += i % 10;
                i /= 10;
            }
            v[g as usize] += 1;
        }
        let mut r = 0;
        let mut m = 0;
        for i in v {
            if m < i {
                m = i;
                r = 1;
            } else if m == i {
                r += 1;
            }
        }
        r
    }
}