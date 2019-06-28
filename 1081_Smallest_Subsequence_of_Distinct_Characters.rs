impl Solution {
    pub fn smallest_subsequence(text: String) -> String {
        let mut count = vec![0; 26];
        let mut visit = vec![0; 26];
        for c in text.chars() {
            let i = (c as u32 - 'a' as u32) as usize;
            count[i] += 1;
        }
        let mut r = String::from("");
        for c in text.chars() {
            let i = (c as u32 - 'a' as u32) as usize;
            count[i] -= 1;
            if visit[i] == 1 {
                continue;
            }
            while !r.is_empty() && c < r.chars().last().unwrap() && count[(r.chars().last().unwrap() as u32 - 'a' as u32) as usize] > 0 {
                visit[(r.chars().last().unwrap() as u32 - 'a' as u32) as usize] = 0;
                r.pop();
            }
            r.push(c);
            visit[i] = 1;
        }
        r
    }
}