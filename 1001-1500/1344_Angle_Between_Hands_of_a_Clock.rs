impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let mut r = minutes as f64 * 6.0 - (hour as f64 * 30.0 + minutes as f64 * 0.5);
        if r < 0.0 {
            r += 360.0;
        }
        if r > 180.0 {
            r = 360.0 - r;
        }
        r
    }
}