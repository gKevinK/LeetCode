impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, x: i32) -> i32 {
        let mut r = 0;
        for i in 0..customers.len() {
            if grumpy[i] == 0 {
                r += customers[i];
            }
        }
        let mut a = 0;
        let mut ma = 0;
        for i in 0..customers.len() {
            a += grumpy[i] * customers[i];
            let l = i as i32 - x;
            if l >= 0 {
                a -= grumpy[l as usize] * customers[l as usize];
            }
            ma = std::cmp::max(ma, a);
        }
        r + ma
    }
}