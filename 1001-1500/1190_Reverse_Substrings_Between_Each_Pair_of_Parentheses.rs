impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut stack = vec![Vec::new()];
        for c in s.chars() {
            if c == '(' {
                stack.push(Vec::new());
            } else if c == ')' {
                let mut l = stack.pop().unwrap();
                let mut t = stack.last_mut().unwrap();
                while let Some(c2) = l.pop() {
                    t.push(c2);
                }
            } else {
                let mut t = stack.last_mut().unwrap();
                t.push(c);
            }
        }
        stack[0].iter().collect()
    }
}