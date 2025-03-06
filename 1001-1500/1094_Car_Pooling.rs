impl Solution {
    pub fn car_pooling(mut trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut count = 0;
        let mut v1 = Vec::new();
        let mut v2 = Vec::new();
        for trip in trips {
            v1.push((trip[1], trip[0]));
            v2.push((trip[2], trip[0]));
        }
        v1.sort();
        v2.sort();
        let mut t: usize = 0;
        for p in v1 {
            while t < v2.len() && v2[t].0 <= p.0 {
                count -= v2[t].1;
                t += 1;
            }
            count += p.1;
            if count > capacity {
                return false;
            }
        }
        true
    }
}