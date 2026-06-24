impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut lines = std::collections::HashMap::new();
        for i in 0..n {
            let (x1, y1) = (points[i][0], points[i][1]);
            for j in (i + 1)..n {
                let (x2, y2) = (points[j][0], points[j][1]);
                let (mut dx, mut dy) = (x2 - x1, y2 - y1);
                let len2 = dx * dx + dy * dy;
                if dx < 0 || (dx == 0 && dy < 0) {
                    (dx, dy) = (-dx, -dy);
                }
                let gcd = Self::gcd(dx.abs(), dy.abs());
                (dx, dy) = (dx / gcd, dy / gcd);
                let x = y1 * dx - x1 * dy;
                lines.entry((dx, dy)).or_insert(Vec::new()).push((x, len2));
            }
        }
        let mut res = 0;
        let mut para = 0;
        let mut len = std::collections::HashMap::new();
        for (_, mut ls) in lines {
            ls.sort_unstable();
            len.clear();
            let mut i = 0;
            while i < ls.len() {
                let mut j = i + 1;
                while j < ls.len() && ls[j].0 == ls[i].0 {
                    j += 1;
                }
                for k in i..j {
                    para += len.get(&ls[k].1).unwrap_or(&0);
                }
                for k in i..j {
                    *len.entry(ls[k].1).or_insert(0) += 1;
                }
                res += (i * (j - i)) as i32;
                i = j;
            }
        }
        res - para / 2
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        if a < b {
            (a, b) = (b, a);
        }
        while b > 0 {
            let t = a % b;
            a = b;
            b = t;
        }
        a
    }
}