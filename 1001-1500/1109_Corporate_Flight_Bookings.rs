impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let mut v = vec![0; n as usize];
        for b in bookings {
            v[b[0] as usize - 1] += b[2];
            if b[1] < n {
                v[b[1] as usize] -= b[2];
            }
        }
        v.iter().scan(0, |st, i| {
            *st += i;
            Some(*st)
        }).collect()
    }
}