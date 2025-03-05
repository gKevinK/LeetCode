impl Solution {
    pub fn min_changes(mut n: i32, mut k: i32) -> i32 {
        let mut res = 0;
        while n != k {
            let n1 = n & 1;
            let k1 = k & 1;
            if n1 != k1 {
                if n1 == 1 {
                    res += 1;
                } else {
                    return -1;
                }
            }
            n >>= 1;
            k >>= 1;
        }
        res
    }
}