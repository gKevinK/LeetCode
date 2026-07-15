impl Solution {
    pub fn gcd_of_odd_even_sums(n: i32) -> i32 {
        let mut sum_odd = 0;
        let mut sum_even = 0;
        for i in 1..=(n * 2) {
            if i % 2 == 1 {
                sum_odd += i;
            } else {
                sum_even += i;
            }
        }
        let mut a = sum_even;
        let mut b = sum_odd;
        while b > 0 {
            let t = a % b;
            a = b;
            b = t;
        }
        a
    }
}