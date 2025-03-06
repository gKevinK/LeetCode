impl Solution {
    fn replace(v: &Vec<i32>, x: i32, y: i32) -> i32 {
        let mut r = 0;
        for &i in v.iter().rev() {
            r = r * 10 + if i == x { y } else { i };
        }
        r
    }
    pub fn max_diff(num: i32) -> i32 {
        let mut v = Vec::new();
        {
            let mut n = num;
            while n > 0 {
                v.push(n % 10);
                n /= 10;
            }
        }
        let mut l = 1;
        for &i in v.iter().rev() {
            if i > 1 {
                l = i;
                break;
            }
        }
        let a = Self::replace(&v, l, if l != *v.last().unwrap() { 0 } else { 1 });
        l = 9;
        for &i in v.iter().rev() {
            if i < 9 {
                l = i;
                break;
            }
        }
        let b = Self::replace(&v, l, 9);
        b - a
    }
}