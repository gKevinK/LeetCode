impl Solution {
    pub fn check_if_prerequisite(n: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut v1 = vec![vec![]; n as usize];
        for p in &prerequisites {
            v1[p[1] as usize].push(p[0]);
        }
        let mut v2 = vec![vec![false; n as usize]; n as usize];
        for i in 0..n {
            Self::f(&mut v1, &mut v2, i);
        }
        queries.iter().map(|q| v2[q[1] as usize][q[0] as usize]).collect()
    }
    
    fn f(mut v1: &mut Vec<Vec<i32>>, mut v2: &mut Vec<Vec<bool>>, i: i32) {
        let v: Vec<_> = v1[i as usize].drain(..).collect();
        for p in v {
            Self::f(&mut v1, &mut v2, p);
            for j in 0..v2.len() {
                v2[i as usize][j] = v2[i as usize][j] | v2[p as usize][j];
            }
            v2[i as usize][p as usize] = true;
        }
    }
}