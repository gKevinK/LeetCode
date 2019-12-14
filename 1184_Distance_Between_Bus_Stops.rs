impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let n = distance.len() as i32;
        let mut l = start;
        let mut r = start;
        let mut dl = 0;
        let mut dr = 0;
        loop {
            if dl <= dr {
                if l == destination {
                    return dl;
                }
                l = (l + n - 1) % n;
                dl += distance[l as usize];
            } else {
                if r == destination {
                    return dr;
                }
                dr += distance[r as usize];
                r = (r + n + 1) % n;
            }
        }
    }
}