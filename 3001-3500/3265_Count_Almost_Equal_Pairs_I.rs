impl Solution {
    fn almost_equal(mut a: i32, mut b: i32) -> bool {
        let mut diff = 0;
        let mut ad = -1;
        let mut bd = -1;
        while a > 0 || b > 0 {
            let a0 = a % 10;
            let b0 = b % 10;
            a /= 10;
            b /= 10;
            if a0 != b0 {
                diff += 1;
                if diff > 2 {
                    return false;
                } else if diff == 1 {
                    ad = a0;
                    bd = b0;
                } else {
                    if a0 != bd || b0 != ad {
                        return false;
                    }
                }
            }
        }
        diff != 1
    }

    pub fn count_pairs(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                count += if Self::almost_equal(nums[i], nums[j]) { 1 } else { 0 };
            }
        }
        count
    }
}