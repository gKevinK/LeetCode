use std::collections::HashMap;

struct UndergroundSystem {
    in_system: HashMap<i32, (String, i32)>,
    m: HashMap<(String, String), (i32, i32)>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl UndergroundSystem {

    fn new() -> Self {
        Self {
            in_system: HashMap::new(),
            m: HashMap::new(),
        }
    }
    
    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.in_system.insert(id, (station_name, t));
    }
    
    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        let r = self.in_system.remove(&id).unwrap();
        let e = self.m.entry((r.0, station_name)).or_insert((0, 0));
        e.0 += t - r.1;
        e.1 += 1;
    }
    
    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let r = self.m.get(&(start_station, end_station)).unwrap();
        r.0 as f64 / r.1 as f64
    }
}

/**
 * Your UndergroundSystem object will be instantiated and called as such:
 * let obj = UndergroundSystem::new();
 * obj.check_in(id, stationName, t);
 * obj.check_out(id, stationName, t);
 * let ret_3: f64 = obj.get_average_time(startStation, endStation);
 */