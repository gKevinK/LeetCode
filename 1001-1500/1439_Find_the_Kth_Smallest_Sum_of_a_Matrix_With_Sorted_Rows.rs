impl Solution {
    pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut v = vec![0];
        for r in mat {
            let mut v2 = Vec::new();
            for i in &v {
                for j in &r {
                    v2.push(*i + *j);
                }
            }
            v2.sort();
            v2.truncate(k as usize);
            v = v2;
        }
        *v.last().unwrap()
    }
}