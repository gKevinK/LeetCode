impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let n = nums.len();
        let mut unique = nums.clone();
        unique.sort_unstable();
        unique.dedup();
        let mut map = vec![0; n];
        for i in 0..n {
            map[i] = unique.partition_point(|&x| x < nums[i]);
        }
        let mut se = [0; 1501];
        let mut so = [0; 1501];
        let mut ce = 0;
        let mut co = 0;
        for i in 0..n {
            se.fill(0);
            so.fill(0);
            ce = 0;
            co = 0;
            for j in i..n {
                let u = map[j];
                if nums[j] % 2 == 0 {
                    if se[u] == 0 {
                        se[u] += 1;
                        ce += 1;
                    }
                } else {
                    if so[u] == 0 {
                        so[u] = 1;
                        co += 1;
                    }
                }
                if ce == co {
                    res = res.max(j - i + 1);
                }
            }
        }
        res as _
    }
}