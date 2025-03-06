impl Solution {
    pub fn min_subsequence(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        let t = (nums.iter().sum::<i32>()) / 2 + 1;
        let mut r = Vec::new();
        let mut i = nums.iter().rev();
        let mut s = 0;
        while s < t {
            let n = *i.next().unwrap();
            s += n;
            r.push(n);
        }
        r
    }
}