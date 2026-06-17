struct TNode {
    child: [usize; 26],
}

impl Solution {
    pub fn partition_string(s: String) -> Vec<String> {
        let sb = s.as_bytes();
        let n = s.len();
        let mut trie = vec![TNode { child: [0; 26] }];
        let mut res = vec![];
        let mut i = 0;
        while i < n {
            let mut j = i;
            let mut t = 0;
            while j < n {
                let c = (sb[j] - b'a') as usize;
                if trie[t].child[c] == 0 {
                    trie[t].child[c] = trie.len();
                    trie.push(TNode { child: [0; 26] });
                    res.push(s[i..=j].to_string());
                    break;
                }
                t = trie[t].child[c];
                j += 1;
            }
            i = j + 1;
        }
        res
    }
}