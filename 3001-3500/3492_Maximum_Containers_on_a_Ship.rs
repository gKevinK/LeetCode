impl Solution {
    pub fn max_containers(n: i32, w: i32, max_weight: i32) -> i32 {
        if max_weight >= n * n * w {
            n * n
        } else {
            max_weight / w
        }
    }
}