impl Solution {
    pub fn best_tower(towers: Vec<Vec<i32>>, center: Vec<i32>, radius: i32) -> Vec<i32> {
        let mut max_q = -1;
        let mut coord = (-1, -1);
        for tower in &towers {
            let dist = (tower[0] - center[0]).abs() + (tower[1] - center[1]).abs();
            if dist <= radius {
                if tower[2] > max_q || (tower[2] == max_q && (tower[0] < coord.0 || (tower[0] == coord.0 && tower[1] < coord.1))) {
                    coord = (tower[0], tower[1]);
                    max_q = tower[2];
                }
            }
        }
        vec![coord.0, coord.1]
    }
}