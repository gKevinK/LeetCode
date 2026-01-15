impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut max_k = 0;
        for i in 0..nums.len() {
            let nu = nums[i] as usize;
            let mut able = vec![false; nu + 1];
            able[nu] = true;

            let mut k = 0;
            while !able[0] {
                if k == queries.len() {
                    return -1;
                }
                if queries[k][0] <= i as i32 && i as i32 <= queries[k][1] {
                    let val = queries[k][2] as usize;
                    for x in val..=nu {
                        able[x - val] |= able[x];
                    }
                }
                k += 1;
            }
            max_k = max_k.max(k as i32);
        }
        max_k
    }
}