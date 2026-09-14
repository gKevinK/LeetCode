impl Solution {
    pub fn climb_stairs(n: i32, costs: Vec<i32>) -> i32 {
        let mut x = (0, 0, 0);
        for i in 0..n as usize {
            x = (x.1, x.2, costs[i] + (x.0 + 9).min(x.1 + 4).min(x.2 + 1));
        }
        x.2
    }
}