impl Solution {
    pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
        let ku = k as usize;
        let mut e = vec![0; ku];
        for i in 0..energy.len() {
            e[i % ku] = e[i % ku].max(0) + energy[i];
        }
        *e.iter().max().unwrap()
    }
}