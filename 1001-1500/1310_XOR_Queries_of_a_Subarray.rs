impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut v = vec![ 0 ];
        v.extend(arr.iter().scan(0, |x, i| {
            *x = *x ^ i;
            Some(*x)
        }));
        queries.iter().map(|q| v[q[0] as usize] ^ v[q[1] as usize + 1]).collect()
    }
}