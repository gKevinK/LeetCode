impl Solution {
    pub fn minimum_moves(s: String) -> i32 {
        let mut moves = 0;
        let mut right = 0;
        for (i, c) in s.chars().enumerate() {
            if c == 'X' {
                if i >= right {
                    right = i + 3;
                    moves += 1;
                }
            }
        }
        moves
    }
}