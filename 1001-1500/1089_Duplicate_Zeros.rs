impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut v = Vec::new();
        for i in 0..arr.len() {
            if i + v.len() + 1 >= arr.len() {
                break;
            }
            if arr[i] == 0 {
                v.push(i);
            }
        }
        for i in (0..arr.len()).rev() {
            let r = (i as u32 - v.len() as u32) as usize;
            if !v.is_empty() && *v.last().unwrap() >= r {
                v.pop();
            }
            arr[i] = arr[r];
        }
    }
}