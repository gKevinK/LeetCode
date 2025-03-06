impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let mut time = 0;
        let tk = tickets[k as usize];
        for i in 0..tickets.len() {
            if i as i32 <= k {
                time += std::cmp::min(tickets[i], tk);
            } else {
                time += std::cmp::min(tickets[i], tk - 1);
            }
        }
        time
    }
}