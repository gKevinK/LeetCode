impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut v = vec![0; 26];
        for c in text.chars() {
            v[c as usize - 'a' as usize] += 1;
        }
        *[ v[0], v[1], v['l' as usize - 'a' as usize] / 2, v['n' as usize - 'a' as usize], v['o' as usize - 'a' as usize] / 2 ].iter().min().unwrap()
    }
}