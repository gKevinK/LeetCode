impl Solution {
    pub fn button_with_longest_time(events: Vec<Vec<i32>>) -> i32 {
        let mut last_index = 0;
        let mut last_time = 0;
        let mut max_index = 0;
        let mut max_time = 0;
        for e in events {
            let time = e[1] - last_time;
            if time > max_time || (time == max_time && e[0] < max_index) {
                max_index = e[0];
                max_time = time;
            }
            last_index = e[0];
            last_time = e[1];
        }
        max_index
    }
}