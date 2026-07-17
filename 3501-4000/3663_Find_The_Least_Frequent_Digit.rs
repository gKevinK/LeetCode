impl Solution {
    pub fn get_least_frequent_digit(n: i32) -> i32 {
        let mut c = [0; 10];
        let mut n = n;
        while n > 0 {
            c[(n % 10) as usize] += 1;
            n /= 10;
        }
        let mut min_i = 0;
        let mut min_v = 100;
        for i in 0..10 {
            if 0 < c[i] && c[i] < min_v {
                min_i = i;
                min_v = c[i];
            }
        }
        min_i as _
    }
}