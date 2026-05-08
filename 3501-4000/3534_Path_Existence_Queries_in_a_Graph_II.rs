impl Solution {
    pub fn path_existence_queries(n: i32, nums: Vec<i32>, max_diff: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let nu = n as usize;
        let mut vnums = Vec::with_capacity(nu);
        for i in 0..nu {
            vnums.push((nums[i], i));
        }
        vnums.sort_unstable();
        let mut vmap = vec![0; nu];
        for i in 0..nu {
            vmap[vnums[i].1] = i;
        }

        let EXP = 17;
        let mut jump = vec![0; nu * EXP];
        let mut r = 0;
        for l in 0..nu {
            while r < nu && vnums[r].0 - vnums[l].0 <= max_diff {
                r += 1;
            }
            jump[l * EXP] = r - 1;
        }
        for j in 1..EXP {
            for i in 0..nu {
                jump[i * EXP + j] = jump[jump[i * EXP + (j - 1)] * EXP + j - 1];
            }
        }

        let mut res = Vec::with_capacity(queries.len());
        for q in queries {
            let (mut a, mut b) = (vmap[q[0] as usize], vmap[q[1] as usize]);
            (a, b) = (a.min(b), a.max(b));
            if a == b {
                res.push(0);
                continue;
            }
            let mut steps = 0;
            for j in (0..EXP).rev() {
                if jump[a * EXP + j] < b {
                    a = jump[a * EXP + j];
                    steps += (1 << j as i32);
                }
            }
            res.push(if jump[a * EXP] >= b { steps + 1 } else { -1 });
        }
        res
    }
}