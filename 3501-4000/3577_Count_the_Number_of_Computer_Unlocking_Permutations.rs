impl Solution {
    pub fn count_permutations(complexity: Vec<i32>) -> i32 {
        let MOD = 1_000_000_007i64;
        let n = complexity.len();
        for i in 1..n {
            if complexity[i] <= complexity[0] {
                return 0;
            }
        }
        let mut res = 1;
        for i in 1..n as i64 {
            res = res * i % MOD;
        }
        res as i32
    }
}