#[derive(Default)]
struct TNode {
    is_end: bool,
    children: [Option<Box<TNode>>; 26]
}

struct StreamChecker {
    chars: Vec<char>,
    trie: TNode
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StreamChecker {

    fn new(words: Vec<String>) -> Self {
        let mut root = TNode::default();
        for w in words {
            let mut node = &mut root;
            for c in w.chars().rev() {
                let idx = c as usize - 'a' as usize;
                node = node.children[idx].get_or_insert_default();
            }
            node.is_end = true;
        }
        Self {
            chars: Vec::with_capacity(40000),
            trie: root
        }
    }

    fn query(&mut self, letter: char) -> bool {
        self.chars.push(letter);
        let mut node = &self.trie;
        for i in (0..self.chars.len()).rev() {
            let idx = self.chars[i] as usize - 'a' as usize;
            if let Some(x) = node.children[idx].as_ref() {
                node = x;
                if node.is_end {
                    return true;
                }
            } else {
                return false;
            }
        }
        false
    }
}

/**
 * Your StreamChecker object will be instantiated and called as such:
 * let obj = StreamChecker::new(words);
 * let ret_1: bool = obj.query(letter);
 */