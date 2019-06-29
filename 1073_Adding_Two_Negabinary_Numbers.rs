impl Solution {
    pub fn add_negabinary(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut i1 = arr1.len() as i32 - 1;
        let mut i2 = arr2.len() as i32 - 1;
        let mut carry = 0;
        let mut sign = 1;
        let mut r = Vec::new();
        while i1 >= 0 || i2 >= 0 || carry != 0 {
            let mut s = carry;
            s += sign * if i1 >= 0 { arr1[i1 as usize] } else { 0 };
            s += sign * if i2 >= 0 { arr2[i2 as usize] } else { 0 };
            r.push((s % 2).abs());
            carry = (s - sign * (s % 2).abs()) / 2;
            i1 -= 1;
            i2 -= 1;
            sign = -sign;
        }
        while r.len() > 1 && *r.last().unwrap() == 0 {
            r.pop();
        }
        r.reverse();
        r
    }
}