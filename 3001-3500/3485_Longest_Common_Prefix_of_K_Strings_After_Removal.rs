struct TNode {
    depth: i32,
    count: i32,
    child: [usize; 26],
}

impl Solution {
    pub fn longest_common_prefix(words: Vec<String>, k: i32) -> Vec<i32> {
        let mut trie = Vec::with_capacity(words.len());
        trie.push(TNode { depth: 0, count: 0, child: [0; 26] });
        let mut max_w_len = 0;
        for w in &words {
            max_w_len = max_w_len.max(w.len());
            let mut i = 0;
            for b in w.as_bytes() {
                let bu = (b - b'a') as usize;
                if trie[i].child[bu] == 0 {
                    trie[i].child[bu] = trie.len();
                    trie.push(TNode { depth: trie[i].depth + 1, count: 0, child: [0; 26] });
                }
                i = trie[i].child[bu];
                trie[i].count += 1;
            }
        }
        let mut max_len = 0;
        let mut k_len = vec![0; max_w_len + 1];
        let mut k_max_len = 0;
        for i in 0..trie.len() {
            let node = &trie[i];
            if node.count >= k + 1 {
                max_len = max_len.max(node.depth);
            } else if node.count == k {
                k_len[node.depth as usize] += 1;
                k_max_len = k_max_len.max(node.depth);
            }
        }

        let mut res = Vec::with_capacity(words.len());
        for w in &words {
            if max_len >= k_max_len {
                res.push(max_len);
                continue;
            }
            let mut len = k_max_len;
            let mut l = k_max_len + 1;
            let mut r = 0;
            let mut i = 0;
            for b in w.as_bytes() {
                let bu = (b - b'a') as usize;
                i = trie[i].child[bu];
                if trie[i].count == k {
                    l = l.min(trie[i].depth);
                    r = r.max(trie[i].depth);
                }
            }
            while len > max_len {
                if k_len[len as usize] > if l <= len && len <= r { 1 } else { 0 } {
                    break;
                }
                len -= 1;
            }
            res.push(len);
        }
        res
    }
}
