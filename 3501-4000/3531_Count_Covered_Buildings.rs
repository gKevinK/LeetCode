impl Solution {
    pub fn count_covered_buildings(n: i32, buildings: Vec<Vec<i32>>) -> i32 {
        let mut vx = vec![(100001, 0); n as usize + 1];
        let mut vy = vec![(100001, 0); n as usize + 1];
        for b in &buildings {
            let x = b[0] as usize;
            let y = b[1] as usize;
            vx[x].0 = vx[x].0.min(y);
            vx[x].1 = vx[x].1.max(y);
            vy[y].0 = vy[y].0.min(x);
            vy[y].1 = vy[y].1.max(x);
        }
        let mut count = 0;
        for b in buildings {
            let x = b[0] as usize;
            let y = b[1] as usize;
            if vx[x].0 < y && y < vx[x].1 && vy[y].0 < x && x < vy[y].1 {
                count += 1;
            }
        }
        count
    }
}