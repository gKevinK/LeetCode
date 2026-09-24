impl Solution {
    pub fn missing_multiple(nums: Vec<i32>, k: i32) -> i32 {
        let mut e = [false; 101];
        for num in nums {
            if num % k == 0 {
                e[(num / k) as usize] = true;
            }
        }
        for i in 1..=100 {
            if e[i] == false {
                return k * (i as i32);
            }
        }
        101
    }
}