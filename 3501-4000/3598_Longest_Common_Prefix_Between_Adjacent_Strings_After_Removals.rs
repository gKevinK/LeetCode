impl Solution {
    pub fn longest_common_prefix(words: Vec<String>) -> Vec<i32> {
        let n = words.len();
        let mut longest = [(0, 0), (0, 0), (0, 0)];
        for i in 0..(n - 1) {
            let w1 = words[i].as_bytes();
            let w2 = words[i + 1].as_bytes();
            let mut len = 0;
            while len < w1.len() && len < w2.len() && w1[len] == w2[len] {
                len += 1;
            }
            if len > longest[2].0 {
                if len > longest[0].0 {
                    longest = [(len, i), longest[0], longest[1]];
                } else if len > longest[1].0 {
                    longest = [longest[0], (len, i), longest[1]];
                } else {
                    longest[2] = (len, i);
                }
            }
        }

        let mut res = vec![0; n];
        for i in 0..n {
            let mut len = 0;
            if i > 0 && i + 1 < n {
                let w1 = words[i - 1].as_bytes();
                let w2 = words[i + 1].as_bytes();
                while len < w1.len() && len < w2.len() && w1[len] == w2[len] {
                    len += 1;
                }
            }
            for j in 0..3 {
                if longest[j].1 + 1 != i && i != longest[j].1 {
                    len = len.max(longest[j].0);
                    break;
                }
            }
            res[i] = len as i32;
        }
        res
    }
}