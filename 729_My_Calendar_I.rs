use std::collections::BTreeMap;

struct MyCalendar {
    m: BTreeMap<i32, i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {

    fn new() -> Self {
        Self { m: BTreeMap::new() }
    }
    
    fn book(&mut self, start: i32, end: i32) -> bool {
        if let Some((k, v)) = self.m.range(start..).next() {
            if k < &end {
                return false;
            }
        }
        if let Some((k, v)) = self.m.range(..start).next_back() {
            if v > &start {
                return false;
            }
        }
        self.m.insert(start, end);
        true
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */