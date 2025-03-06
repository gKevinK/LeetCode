impl Solution {
    fn rank(i: i32, n: i32) -> i32 {
        if n % 2 == 0 {
            (2 * i + 1) % (n + 1)
        } else {
            (2 * i) % n
        }
    }
    pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
        let n = barcodes.len() as i32;
        let mut m = std::collections::HashMap::new();
        for &b in &barcodes {
            let e = m.entry(b).or_insert(0);
            *e += 1;
        }
        let mut mb = 0;
        let mut mc = 0;
        for (&b, &c) in &m {
            if c > mc {
                mb = b;
                mc = c;
            }
        }
        let mut i = 0;
        let mut r = vec![0; barcodes.len()];
        for j in 0..mc {
            r[Self::rank(i, n) as usize] = mb;
            i += 1;
        }
        for (&b, &c) in &m {
            if b == mb {
                continue;
            }
            for j in 0..c {
                r[Self::rank(i, n) as usize] = b;
                i += 1;
            }
        }
        r
    }
}