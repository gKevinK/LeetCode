impl Solution {
    pub fn find_min_fibonacci_numbers(mut k: i32) -> i32 {
        let mut v = vec![1, 1];
        while *v.last().unwrap() < k {
            v.push(v[v.len() - 2] + v[v.len() - 1]);
        }
        let mut r = 0;
        let mut i = v.len();
        while k > 0 {
            i -= 1;
            if v[i] <= k {
                k -= v[i];
                r += 1;
            }
        }
        r
    }
}