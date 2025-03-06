struct TweetCounts {
    t: std::collections::HashMap<String, Vec<i32>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TweetCounts {

    fn new() -> Self {
        TweetCounts {
            t: std::collections::HashMap::new(),
        }
    }
    
    fn record_tweet(&mut self, tweet_name: String, time: i32) {
        self.t.entry(tweet_name).or_insert(vec![]).push(time);
    }
    
    fn get_tweet_counts_per_frequency(&mut self, freq: String, tweet_name: String, mut start_time: i32, end_time: i32) -> Vec<i32> {
        let mut temp = vec![];
        let mut v = self.t.get_mut(&tweet_name).unwrap_or(&mut temp);
        v.sort_unstable();
        let freq2: &str = &freq;
        let p = match freq2 {
            "minute" => 60, "hour" => 3600, "day" => 86400, _ => 1
        };
        let mut r = Vec::new();
        let mut i = v.binary_search(&start_time).unwrap_or_else(|x| x);
        while i > 0 && v[i - 1] == start_time { i -= 1; }
        while start_time <= end_time {
            let mut num = 0;
            while i < v.len() && v[i] < start_time + p && v[i] <= end_time {
                num += 1;
                i += 1;
            }
            r.push(num);
            start_time += p;
        }
        r
    }
}

/**
 * Your TweetCounts object will be instantiated and called as such:
 * let obj = TweetCounts::new();
 * obj.record_tweet(tweetName, time);
 * let ret_2: Vec<i32> = obj.get_tweet_counts_per_frequency(freq, tweetName, startTime, endTime);
 */