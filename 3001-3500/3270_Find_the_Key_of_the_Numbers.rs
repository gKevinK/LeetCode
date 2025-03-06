impl Solution {
    pub fn generate_key(num1: i32, num2: i32, num3: i32) -> i32 {
        let mut v = [9; 4];
        for num in [num1, num2, num3] {
            let mut n = num;
            for i in 0..4 {
                let d = n % 10;
                v[i] = std::cmp::min(v[i], d);
                n /= 10;
            }
        }
        ((v[3] * 10 + v[2]) * 10 + v[1]) * 10 + v[0]
    }
}