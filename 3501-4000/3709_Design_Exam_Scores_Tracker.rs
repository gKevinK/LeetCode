struct ExamTracker {
    data: Vec<(i32, i64)>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ExamTracker {

    fn new() -> Self {
        Self {
            data: vec![(0, 0)],
        }
    }
    
    fn record(&mut self, time: i32, score: i32) {
        let last = self.data.last().unwrap().1;
        self.data.push((time, last + score as i64));
    }
    
    fn total_score(&self, start_time: i32, end_time: i32) -> i64 {
        let lo = self.data.partition_point(|&t| t.0 < start_time);
        let hi = self.data.partition_point(|&t| t.0 <= end_time);
        self.data[hi - 1].1 - self.data[lo - 1].1
    }
}

/**
 * Your ExamTracker object will be instantiated and called as such:
 * let obj = ExamTracker::new();
 * obj.record(time, score);
 * let ret_2: i64 = obj.total_score(startTime, endTime);
 */