impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let mut count = vec![ 0; 100_001 ];
        for i in &arr {
            count[*i as usize] += 1;
        }
        let mut v = Vec::new();
        for i in count {
            if i > 0 {
                v.push(i);
            }
        }
        v.sort();
        v.reverse();
        let mut c = 0;
        for i in 0..v.len() {
            c += v[i];
            if c >= arr.len() / 2 {
                return i as i32 + 1;
            }
        }
        0
    }
}