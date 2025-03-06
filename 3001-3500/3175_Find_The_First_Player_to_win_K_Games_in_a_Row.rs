impl Solution {
    pub fn find_winning_player(skills: Vec<i32>, k: i32) -> i32 {
        let mut last = 0;
        let mut wins = 0;
        for i in 1..skills.len() {
            if skills[last] > skills[i] {
                wins += 1;
            } else {
                last = i;
                wins = 1;
            }
            if wins >= k {
                return last as i32;
            }
        }
        last as i32
    }
}