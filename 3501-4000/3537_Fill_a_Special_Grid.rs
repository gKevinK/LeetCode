impl Solution {
    pub fn special_grid(n: i32) -> Vec<Vec<i32>> {
        let w = 1 << n as usize;
        let mut res = vec![vec![0; w]; w];
        Self::fill(&mut res, 0, 0, w, 0);
        res
    }

    fn fill(mut g: &mut Vec<Vec<i32>>, x: usize, y: usize, size: usize, start: i32) {
        if size == 1 {
            g[x][y] = start;
        } else {
            let quad_num = (size * size / 4) as i32;
            Self::fill(&mut g, x, y + size / 2, size / 2, start);
            Self::fill(&mut g, x + size / 2, y + size / 2, size / 2, start + quad_num);
            Self::fill(&mut g, x + size / 2, y, size / 2, start + quad_num * 2);
            Self::fill(&mut g, x, y, size / 2, start + quad_num * 3);
        }
    }
}