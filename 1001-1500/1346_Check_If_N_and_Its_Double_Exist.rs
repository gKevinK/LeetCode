impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut s = std::collections::HashSet::new();
        for i in arr {
            if s.contains(&i) {
                return true;
            }
            s.insert(i * 2);
            if i % 2 == 0 {
                s.insert(i / 2);
            }
        }
        false
    }
}