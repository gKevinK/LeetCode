impl Solution {
    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut v = vec![false; 2048];
        for i in 0..n {
            for j in i..n {
                v[(nums[i] ^ nums[j]) as usize] = true;
            }
        }
        let mut s2 = Vec::with_capacity(v.len());
        for i in 0..v.len() {
            if v[i] {
                s2.push(i as i32);
            }
        }
        v.fill(false);
        for i in 0..nums.len() {
            for j in 0..s2.len() {
                v[(nums[i] ^ s2[j]) as usize] = true;
            }
        }
        v.iter().fold(0, |s, e| s + *e as i32)
    }
}