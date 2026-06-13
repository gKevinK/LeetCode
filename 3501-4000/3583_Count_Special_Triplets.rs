impl Solution {
    pub fn special_triplets(nums: Vec<i32>) -> i32 {
        let mut s1 = [0; 100001];
        let mut s2 = [0; 100001];
        for &num in &nums {
            s2[num as usize] += 1;
        }
        const MOD: i64 = 1_000_000_007;
        let mut res = 0;
        for &num in &nums {
            s2[num as usize] -= 1;
            let n2 = num as usize * 2;
            if n2 < s1.len() {
                res = (res + s1[n2] * s2[n2]) % MOD;
            }
            s1[num as usize] += 1;
        }
        res as _
    }
}