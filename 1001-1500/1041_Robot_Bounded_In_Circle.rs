impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let mut t = 0;
        let mut d = (0, 1);
        let mut l = (0, 0);
        for c in instructions.chars() {
            if c == 'L' {
                t -= 1;
                d = (-d.1, d.0);
            } else if c == 'R' {
                t += 1;
                d = (d.1, -d.0);
            } else {
                l = (l.0 + d.0, l.1 + d.1);
            }
        }
        l == (0, 0) || d != (0, 1)
    }
}