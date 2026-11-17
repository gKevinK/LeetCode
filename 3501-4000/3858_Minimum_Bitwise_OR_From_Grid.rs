impl Solution {
    pub fn minimum_or(grid: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut zero = 0;
        let m = grid.len();
        for b in (0..=16).rev() {
            let test = zero | (1 << b);
            let mut ok = true;
            for x in 0..m {
                let mut row_ok = false;
                for &num in &grid[x] {
                    if (num & test) == 0 {
                        row_ok = true;
                        break;
                    }
                }
                if !row_ok {
                    ok = false;
                    break;
                }
            }
            if ok {
                zero = test;
            } else {
                res |= 1 << b;
            }
        }
        res
    }
}