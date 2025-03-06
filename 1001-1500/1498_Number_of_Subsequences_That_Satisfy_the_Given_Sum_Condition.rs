impl Solution {
    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let MOD = 1_000_000_007;
        let mut v = vec![0, 1];
        while v.len() <= nums.len() {
            v.push(*v.last().unwrap() * 2 % MOD);
        }
        let mut r = 0;
        let mut i = 0;
        let mut j = nums.len();
        while i < j {
            while j > 0 && nums[i] + nums[j - 1] > target {
                j -= 1;
            }
            if i >= j {
                break;
            }
            r = (r + v[j - i]) % MOD;
            i += 1;
        }
        r
    }
}