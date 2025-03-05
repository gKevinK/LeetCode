impl Solution {
    pub fn losing_player(mut x: i32, mut y: i32) -> String {
        loop {
            x -= 1;
            y -= 4;
            if x < 0 || y < 0 {
                return "Bob".to_string();
            }
            x -= 1;
            y -= 4;
            if x < 0 || y < 0 {
                return "Alice".to_string();
            }
        }
    }
}