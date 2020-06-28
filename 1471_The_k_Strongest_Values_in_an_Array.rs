impl Solution {
    pub fn get_strongest(mut arr: Vec<i32>, k: i32) -> Vec<i32> {
        arr.sort();
        let m = arr[(arr.len() - 1) / 2];
        arr.sort_by_key(|i| (-(i - m).abs(), -i));
        arr.resize(k as usize, 0);
        arr
    }
}