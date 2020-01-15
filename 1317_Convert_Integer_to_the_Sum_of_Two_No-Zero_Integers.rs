impl Solution {
    pub fn get_no_zero_integers(mut n: i32) -> Vec<i32> {
        let mut i = 0;
        let mut m = 1;
        let mut a = 0;
        let mut b = 0;
        while n > 0 {
            let mut d = n % 10;
            n /= 10;
            let mut ad = 1;
            if d < 2 {
                if n > 0 {
                    d += 10;
                    n -= 1;
                    if d == 11 {
                        ad = 2;
                    }
                } else {
                    ad = 0;
                }
            }
            a += ad * m;
            b += (d - ad) * m;
            m *= 10;
            
        }
        vec![a, b]
    }
}