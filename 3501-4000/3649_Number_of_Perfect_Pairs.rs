impl Solution {
    pub fn perfect_pairs(mut nums: Vec<i32>) -> i64 {
        let n = nums.len();
        nums.iter_mut().for_each(|x| *x = x.abs());
        nums.sort_unstable();
        let mut res = 0;
        let mut i = 0;
        let mut j = 0;
        while i < n {
            j = j.max(i);
            while j + 1 < n && nums[i] * 2 >= nums[j + 1] {
                j += 1;
            }
            res += (j - i) as i64;
            i += 1;
        }
        res
    }
}