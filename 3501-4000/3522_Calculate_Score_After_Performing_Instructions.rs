impl Solution {
    pub fn calculate_score(instructions: Vec<String>, values: Vec<i32>) -> i64 {
        let n = instructions.len();
        let mut res = 0;
        let mut visit = vec![false; n];
        let mut i = 0;
        while i < n && visit[i] == false {
            visit[i] = true;
            if &instructions[i] == "add" {
                res += values[i] as i64;
                i += 1;
            } else {
                i = (i as i32 + values[i]) as usize;
            }
        }
        res
    }
}