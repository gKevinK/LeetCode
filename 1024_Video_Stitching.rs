impl Solution {
    pub fn video_stitching(clips: Vec<Vec<i32>>, t: i32) -> i32 {
        let mut cs = clips.clone();
        let mut s: Vec<i32> = Vec::new();
        let mut last = 0;
        cs.sort_by(|a, b| if a[1] != b[1] { a[1].cmp(&b[1]) } else { b[0].cmp(&a[0]) });
        for c in cs {
            if c[0] > last {
                continue;
            }
            let mut l2 = last;
            while let Some(&l) = s.last() {
                if l < c[0] {
                    break;
                }
                l2 = l;
                s.pop();
            }
            s.push(l2);
            last = c[1];
        }
        if last < t {
            return -1;
        }
        while let Some(&l) = s.last() {
            if l < t {
                break;
            }
            last = l;
            s.pop();
        }
        s.len() as i32
    }
}