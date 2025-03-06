impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut r = 0;
        for n in nums {
            let root = (n as f64).sqrt() as i32;
            let mut d1 = -1;
            let mut d2 = n;
            for i in 2..=root {
                if d1 == -1 && n % i == 0 {
                    if n == i * i {
                        break;
                    }
                    d1 = i;
                    d2 = n / i;
                } else if d1 != -1 && n % i == 0 {
                    d1 = -1;
                    break;
                }
            }
            if d1 != -1 {
                r += 1 + d1 + d2 + n;
            }
        }
        r
    }
}