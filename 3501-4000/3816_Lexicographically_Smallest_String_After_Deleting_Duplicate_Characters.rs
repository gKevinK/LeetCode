impl Solution {
    pub fn lex_smallest_after_deletion(s: String) -> String {
        let mut freq = [0; 26];
        let bytes = s.as_bytes();
        for &b in bytes {
            freq[(b - b'a') as usize] += 1;
        }
        let mut stack = Vec::new();
        let mut in_stack = [0; 26];
        for &b in bytes {
            let i = (b - b'a') as usize;
            freq[i] -= 1;
            while let Some(&top) = stack.last() {
                let top_i = (top - b'a') as usize;
                if top > b && (freq[top_i] > 0 || in_stack[top_i] > 1) {
                    stack.pop();
                    in_stack[top_i] -= 1;
                } else {
                    break;
                }
            }
            stack.push(b);
            in_stack[i] += 1;
        }
        while let Some(&top) = stack.last() {
            let top_i = (top - b'a') as usize;
            if in_stack[top_i] > 1 {
                stack.pop();
                in_stack[top_i] -= 1;
            } else {
                break;
            }
        }
        String::from_utf8(stack).unwrap()
    }
}