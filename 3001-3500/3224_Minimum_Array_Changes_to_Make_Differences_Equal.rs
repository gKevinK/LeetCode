impl Solution {
    pub fn min_changes(nums: Vec<i32>, k: i32) -> i32 {
        let mut v = vec![0; k as usize + 2];
        let n = nums.len();
        for i in 0..(n / 2) {
            let a = nums[i];
            let b = nums[n - i - 1];
            let abs = (a - b).abs();
            let max = std::cmp::max(a.max(b), k - a.min(b));
            v[0] += 1;
            v[abs as usize] -= 1;
            v[abs as usize + 1] += 1;
            v[max as usize + 1] += 1;
        }
        let mut ans = n as i32 / 2;
        let mut sum = 0;
        for c in v {
            sum += c;
            ans = ans.min(sum);
        }
        ans
    }
}