impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let mut start = 0;
        let mut end = 0;
        let mut sum = 0;
        let mut r = 0;
        while end < k as usize {
            sum += arr[end];
            end += 1;
        }
        if sum >= threshold * k {
            r += 1;
        }
        while end < arr.len() {
            sum += arr[end];
            end += 1;
            sum -= arr[start];
            start += 1;
            if sum >= threshold * k {
                r += 1;
            }
        }
        r
    }
}