impl Solution {
    pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
        let mut v = vec![a, b, c];
        v.sort();
        let mut x = if v[2] - v[1] == 2 || v[1] - v[0] == 2 { 1 } else {
            (v[2] - v[1] != 1) as i32 + (v[1] - v[0] != 1) as i32 };
        vec![x, v[2] - v[0] - 2]
    }
}