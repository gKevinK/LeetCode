impl Solution {
    pub fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
        let mut l = 0;
        while l + ((m * k) as usize) <= arr.len() {
            let mut f = true;
            for i in 0..k as usize{
                for j in 0..m as usize {
                    if arr[l + j] != arr[l + m as usize * i + j] {
                        f = false;
                        break;
                    }
                }
            }
            if f {
                return true;
            }
            l += 1;
        }
        false
    }
}