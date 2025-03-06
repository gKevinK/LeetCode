impl Solution {
    pub fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
        let mut r = 0i64;
        let l = nums.len();
        for i in 0..(l / 2) {
            let mut n1 = nums[i] as i64;
            let mut n2 = nums[l - i - 1];
            while n2 > 0 {
                n2 /= 10;
                n1 *= 10;
            }
            r += n1 + nums[l - i - 1] as i64;
        }
        if l % 2 == 1 {
            r += nums[l as usize / 2] as i64;
        }
        r
    }
}