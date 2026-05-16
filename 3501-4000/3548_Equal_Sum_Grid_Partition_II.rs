impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        const LIMIT: i32 = 100000;
        let m = grid.len();
        let n = grid[0].len();
        if m == 1 || n == 1 {
            let mut flat = vec![0; m * n];
            let mut total = 0;
            for x in 0..m {
                for y in 0..n {
                    flat[x * n + y] = grid[x][y];
                    total += grid[x][y] as i64;
                }
            }
            let mut seen = [0u8; LIMIT as usize + 1];
            let mut sum2 = 0;
            let mut maxv = 0;
            for i in 0..(m * n - 1) {
                sum2 += flat[i] as i64 * 2;
                maxv = maxv.max(flat[i]);
                seen[flat[i] as usize] = 1;
                if sum2 < total {
                    continue;
                } else if sum2 == total {
                    return true;
                } else {
                    let d = (sum2 - total) as i32;
                    if d > maxv {
                        break;
                    }
                    if seen[d as usize] == 1 {
                        if flat[0] == d || flat[i] == d {
                            return true;
                        }
                    }
                }
            }
            let mut sum2 = 0;
            let mut maxv = 0;
            for i in (1..(m * n)).rev() {
                sum2 += flat[i] as i64 * 2;
                maxv = maxv.max(flat[i]);
                seen[flat[i] as usize] = 2;
                if sum2 < total {
                    continue;
                } else if sum2 == total {
                    return true;
                } else {
                    let d = (sum2 - total) as i32;
                    if d > maxv {
                        break;
                    }
                    if seen[d as usize] == 2 {
                        if flat[i] == d || flat[m * n - 1] == d {
                            return true;
                        }
                    }
                }
            }
            return false;
        }

        let mut total = 0;
        let mut rsum = vec![0; m];
        let mut csum = vec![0; n];
        for x in 0..m {
            for y in 0..n {
                let v = grid[x][y] as i64;
                total += v;
                rsum[x] += v;
                csum[y] += v;
            }
        }

        let mut sum2 = 0;
        let mut seen = [0u8; LIMIT as usize + 1];
        let mut maxv = 0;
        for x in 0..(m - 1) {
            sum2 += rsum[x] * 2;
            for y in 0..n {
                seen[grid[x][y] as usize] = 1;
                maxv = maxv.max(grid[x][y]);
            }
            if sum2 < total {
                continue;
            } else if sum2 == total {
                return true;
            } else {
                let d = (sum2 - total) as i32;
                if d > maxv {
                    break;
                }
                if seen[d as usize] == 1 {
                    if x > 0 || grid[0][0] == d || grid[0][n - 1] == d {
                        return true;
                    }
                }
            }
        }
        sum2 = 0;
        maxv = 0;
        for x in (1..m).rev() {
            sum2 += rsum[x] * 2;
            for y in 0..n {
                seen[grid[x][y] as usize] = 2;
                maxv = maxv.max(grid[x][y]);
            }
            if sum2 < total {
                continue;
            } else if sum2 == total {
                return true;
            } else {
                let d = (sum2 - total) as i32;
                if d > maxv {
                    break;
                }
                if seen[d as usize] == 2 {
                    if x < m - 1 || grid[m - 1][0] == d || grid[m - 1][n - 1] == d {
                        return true;
                    }
                }
            }
        }
        sum2 = 0;
        maxv = 0;
        for y in 0..n - 1 {
            sum2 += csum[y] * 2;
            for x in 0..m {
                seen[grid[x][y] as usize] = 3;
                maxv = maxv.max(grid[x][y]);
            }
            if sum2 < total {
                continue;
            } else if sum2 == total {
                return true;
            } else {
                let d = (sum2 - total) as i32;
                if d > maxv {
                    break;
                }
                if seen[d as usize] == 3 {
                    if y > 0 || grid[0][0] == d || grid[m - 1][0] == d {
                        return true;
                    }
                }
            }
        }
        sum2 = 0;
        maxv = 0;
        for y in (1..n).rev() {
            sum2 += csum[y] * 2;
            for x in 0..m {
                seen[grid[x][y] as usize] = 4;
                maxv = maxv.max(grid[x][y]);
            }
            if sum2 < total {
                continue;
            } else if sum2 == total {
                return true;
            } else {
                let d = (sum2 - total) as i32;
                if d > maxv {
                    break;
                }
                if seen[d as usize] == 4 {
                    if y < n - 1 || grid[0][n - 1] == d || grid[m - 1][n - 1] == d {
                        return true;
                    }
                }
            }
        }
        false
    }
}