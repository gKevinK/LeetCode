impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let mut at = 0;
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if nums[j] - nums[i] != diff || j + 1 >= nums.len() {
                    continue;
                }
                let nk = nums[j] + diff;
                let mut l = j + 1;
                let mut r = nums.len();
                while l + 1 < r {
                    let m = (r - l) / 2 + l;
                    if nums[m] < nk {
                        l = m;
                    } else if nums[m] > nk {
                        r = m;
                    } else {
                        l = m;
                        r = m + 1;
                        break;
                    }
                }
                // println!("{}:{}, {}:{}, {}:{}, {}", i, nums[i], j, nums[j], l, nums[l], nk);
                if nums[l] == nk {
                    at += 1;
                }
            }
        }
        at
    }
}