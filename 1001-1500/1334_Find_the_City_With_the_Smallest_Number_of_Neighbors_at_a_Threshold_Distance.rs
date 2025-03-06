impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        use std::cmp::min;

        let n2 = n as usize;
        let mut distance = vec![vec![1_000_000_000; n2]; n2];
        for i in 0..n2 {
            distance[i][i] = 0;
        }
        for e in &edges {
            distance[e[0] as usize][e[1] as usize] = e[2];
            distance[e[1] as usize][e[0] as usize] = e[2];
        }
        for k in 0..n2 {
            for i in 0..n2 {
                for j in i..n2 {
                    distance[i][j] = min(distance[i][j], distance[i][k] + distance[k][j]);
                    distance[j][i] = distance[i][j];
                }
            }
        }
        let mut city = -1;
        let mut num = n;
        for i in 0..n2 {
            let mut c = -1;
            for j in 0..n2 {
                if distance[i][j] <= distance_threshold {
                    c += 1;
                }
            }
            if c <= num {
                num = c;
                city = i as i32;
            }
        }
        city
    }
}