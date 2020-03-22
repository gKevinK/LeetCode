impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_by_cached_key(|i| {
            let mut i2 = *i;
            let mut n = 0;
            while i2 > 0 {
                n += i2 & 1;
                i2 >>= 1;
            }
            (n, *i)
        });
        arr
    }
}