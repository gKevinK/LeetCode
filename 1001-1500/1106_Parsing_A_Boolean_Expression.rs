impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let mut stack: Vec<(char, bool)> = Vec::new();
        let mut t = 'x';
        let mut b = false;
        stack.push(('x', false));
        for ch in expression.chars() {
            match ch {
                '&' => {
                    stack.push((t, b));
                    t = ch;
                    b = true;
                },
                '|' => {
                    stack.push((t, b));
                    t = ch;
                    b = false;
                },
                '!' => {
                    stack.push((t, b));
                    t = ch;
                    b = false;
                },
                '(' => {},
                ')' => {
                    let a = stack.pop().unwrap();
                    let nb = if t == '!' { !b } else { b };
                    t = a.0;
                    b = a.1;
                    if t == '&' {
                        b = b & nb;
                    } else if t == '|' {
                        b = b | nb;
                    } else {
                        b = nb;
                    }
                },
                't' | 'f' => {
                    let nb = (ch == 't');
                    if t == '&' {
                        b = b & nb;
                    } else if t == '|' {
                        b = b | nb;
                    } else {
                        b = nb;
                    }
                },
                ',' | _ => {},
            }
        }
        b
    }
}