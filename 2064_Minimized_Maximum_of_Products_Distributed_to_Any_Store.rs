impl Solution {
    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        let max = *quantities.iter().max().unwrap();
        let mut lo = 1;
        let mut hi = max + 1;
        while lo + 1 < hi {
            let mi = (hi - lo - 1) / 2 + lo;
            let mut need = 0;
            for q in &quantities {
                need += (*q + mi - 1) / mi;
            }
            if need > n {
                lo = mi + 1;
            } else {
                hi = mi + 1;
            }
        }
        lo
    }
}