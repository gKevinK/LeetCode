impl Solution {
    pub fn count_arrays(original: Vec<i32>, bounds: Vec<Vec<i32>>) -> i32 {
        let mut b1 = i32::MIN;
        let mut b2 = i32::MAX;
        for i in 0..original.len() {
            b1 = b1.max(bounds[i][0] - original[i]);
            b2 = b2.min(bounds[i][1] - original[i]);
        }
        0.max(b2 - b1 + 1)
    }
}