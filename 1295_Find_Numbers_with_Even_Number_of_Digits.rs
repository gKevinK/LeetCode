impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        nums.iter().filter(|&_n| {
            let mut n = *_n;
            let mut l = 0;
            while n > 0 {
                n /= 10;
                l += 1;
            }
            l % 2 == 0
        }).count() as i32
    }
}