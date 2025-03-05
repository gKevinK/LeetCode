impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort();
        for i in 2..arr.len() {
            if arr[i] - arr[i - 1] != arr[i - 1] - arr[i - 2] {
                return false;
            }
        }
        true
    }
}