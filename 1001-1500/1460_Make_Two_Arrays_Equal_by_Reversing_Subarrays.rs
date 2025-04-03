impl Solution {
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        let mut v = [0; 1001];
        for i in &target {
            v[*i as usize] += 1;
        }
        for i in &arr {
            v[*i as usize] -= 1;
            if v[*i as usize] < 0 {
                return false;
            }
        }
        true
    }
}