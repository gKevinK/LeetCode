impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut r = 0;
        let mut vc = vec![0; 26];
        for c in chars.chars() {
            vc[c as usize - 'a' as usize] += 1;
        }
        for s in &words {
            let mut vc2 = vc.clone();
            let mut good = true;
            for c in s.chars() {
                vc2[c as usize - 'a' as usize] -= 1;
                if vc2[c as usize - 'a' as usize] < 0 {
                    good = false;
                    break;
                }
            }
            if good {
                r += s.len() as i32;
            }
        }
        r
    }
}