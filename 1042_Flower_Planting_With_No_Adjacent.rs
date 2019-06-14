impl Solution {
    pub fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32> {
        let mut g: Vec<Vec<usize>> = vec![Vec::new(); n as usize];
        let mut r: Vec<i32> = vec![0; n as usize];
        for p in paths {
            g[(p[0] - 1) as usize].push((p[1] - 1) as usize);
            g[(p[1] - 1) as usize].push((p[0] - 1) as usize);
        }
        for i in 0..n as usize {
            let mut c = [0; 5];
            for &ii in &g[i] {
                c[r[ii] as usize] = 1;
            }
            for ii in 1..5 {
                if c[ii] == 0 {
                    r[i] = ii as i32;
                    break;
                }
            }
        }
        r
    }
}