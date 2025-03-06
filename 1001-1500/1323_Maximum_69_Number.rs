impl Solution {
    pub fn maximum69_number (mut num: i32) -> i32 {
        let mut change = false;
        let mut v = Vec::new();
        while num > 0 {
            v.push(num % 10);
            num /= 10;
        }
        let mut r = 0;
        while let Some(mut l) = v.pop() {
            if !change && l == 6 {
                l = 9;
                change = true;
            }
            r *= 10;
            r += l;
        }
        r
    }
}