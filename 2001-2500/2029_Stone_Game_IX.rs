impl Solution {
    pub fn stone_game_ix(stones: Vec<i32>) -> bool {
        let mut m = vec![0i32; 3];
        for value in stones {
            m[(value % 3) as usize] += 1;
        }
        if m[0] % 2 == 0 {
            return m[1] > 0 && m[2] > 0;
        }
        i32::abs(m[1] - m[2]) > 2
    }
}