impl Solution {
    fn f(arr: &Vec<i32>, sum: &Vec<i32>, k: i32) -> i32 {
        let mut lo = 0;
        let mut hi = arr.len();
        while lo < hi {
            let mi = (hi - lo) / 2 + lo;
            if arr[mi] < k {
                lo = mi + 1;
            } else {
                hi = mi;
            }
        }
        (if lo > 0 { sum[lo - 1] } else { 0 }) + (arr.len() - lo) as i32 * k
    }

    pub fn find_best_value(mut arr: Vec<i32>, target: i32) -> i32 {
        arr.sort();
        let mut sum: Vec<i32> = arr.iter().scan(0, |s, &i| {
            *s = *s + i;
            Some(*s)
        }).collect();
        let mut lo = 0;
        let mut hi = *arr.last().unwrap();
        while lo < hi {
            let mi = (hi - lo) / 2 + lo;
            let s = Self::f(&arr, &sum, mi);
            if s < target {
                lo = mi + 1;
            } else if s == target {
                return mi;
            } else {
                hi = mi;
            }
        }
        if target - Self::f(&arr, &sum, lo - 1) <= Self::f(&arr, &sum, lo) - target { lo - 1 } else { lo }
    }
}