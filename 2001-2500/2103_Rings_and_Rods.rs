impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let mut color = vec![vec![0; 3]; 10];
        let r: Vec<_> = rings.chars().collect();
        for i in 0..(r.len() / 2) {
            let pos = r[i * 2 + 1] as usize - '0' as usize;
            match r[i * 2] {
                'R' => { color[pos][0] = 1 },
                'G' => { color[pos][1] = 1 },
                'B' => { color[pos][2] = 1 },
                _ => {},
            };
        }
        color.iter().filter(|x| x[0] + x[1] + x[2] == 3).count() as i32
    }
}