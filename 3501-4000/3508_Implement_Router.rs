use std::collections::{HashSet, HashMap, VecDeque};

struct Router {
    limit: usize,
    packets: VecDeque<(i32, i32, i32)>,
    time: i32,
    set: HashSet<(i32, i32)>,
    dest: HashMap<i32, VecDeque<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Router {

    fn new(memoryLimit: i32) -> Self {
        Router {
            limit: memoryLimit as usize,
            packets: VecDeque::new(),
            time: 0,
            set: HashSet::new(),
            dest: HashMap::new(),
        }
    }
    
    fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        if timestamp == self.time && self.set.contains(&(source, destination)) {
            return false;
        }
        if timestamp > self.time {
            self.time = timestamp;
            self.set.clear();
        }
        if self.packets.len() == self.limit {
            self.forward_packet();
        }
        self.packets.push_back((source, destination, timestamp));
        self.set.insert((source, destination));
        self.dest.entry(destination).or_insert_with(VecDeque::new).push_back(timestamp);
        true
    }
    
    fn forward_packet(&mut self) -> Vec<i32> {
        if let Some((s, d, t)) = self.packets.pop_front() {
            if t == self.time {
                self.set.remove(&(s, d));
            }
            self.dest.entry(d).and_modify(|q| { q.pop_front(); });
            vec![s, d, t]
        } else {
            vec![]
        }
    }
    
    fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        if let Some(v) = self.dest.get(&destination) {
            let a = v.partition_point(|&x| x < start_time);
            let b = v.partition_point(|&x| x <= end_time);
            (b - a) as i32
        } else {
            0
        }
    }
}

/**
 * Your Router object will be instantiated and called as such:
 * let obj = Router::new(memoryLimit);
 * let ret_1: bool = obj.add_packet(source, destination, timestamp);
 * let ret_2: Vec<i32> = obj.forward_packet();
 * let ret_3: i32 = obj.get_count(destination, startTime, endTime);
 */