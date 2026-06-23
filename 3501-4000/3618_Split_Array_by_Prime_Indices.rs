const LIMIT: usize = 100001;

const PRIME: [bool; LIMIT] = {
    let mut p = [true; LIMIT];
    p[0] = false;
    p[1] = false;
    let mut i = 2;
    while i * i <= LIMIT {
        let mut j = i * i;
        while j < LIMIT {
            p[j] = false;
            j += i;
        }
        i += 1;
    }
    p
};

impl Solution {
    pub fn split_array(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut sum_a = 0;
        let mut sum_b = 0;
        for i in 0..n {
            if PRIME[i] {
                sum_a += nums[i] as i64;
            } else {
                sum_b += nums[i] as i64;
            }
        }
        (sum_a - sum_b).abs()
    }
}