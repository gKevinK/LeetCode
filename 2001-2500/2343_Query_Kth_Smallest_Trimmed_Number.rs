impl Solution {
    pub fn smallest_trimmed_numbers(nums: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = vec![];
        for q in queries {
            let k = q[0];
            let trim = q[1];
            let mut v = vec![];
            for (i, n) in nums.iter().enumerate() {
                let trimmed = String::from(&n[(n.len() - trim as usize)..]);
                v.push((trimmed, i));
            }
            v.sort();
            res.push(v[(k - 1) as usize].1 as i32);
        }
        res
    }
}