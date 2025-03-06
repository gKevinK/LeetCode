impl Solution {
    pub fn alphabet_board_path(target: String) -> String {
        let t: Vec<char> = target.chars().collect();
        let mut r = String::from("");
        let mut p = (0, 0);
        for c in t {
            let p2 = ((c as i32 - 'a' as i32) / 5, (c as i32 - 'a' as i32) % 5);
            while p != p2 {
                if p.0 < 5 && p.1 != p2.1 {
                    if p2.1 > p.1 {
                        p.1 += 1;
                        r.push('R');
                    } else {
                        p.1 -= 1;
                        r.push('L');
                    }
                } else {
                    if p2.0 > p.0 {
                        p.0 += 1;
                        r.push('D');
                    } else {
                        p.0 -= 1;
                        r.push('U');
                    }
                }
            }
            r.push('!');
        }
        r
    }
}