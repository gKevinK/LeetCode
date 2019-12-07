use std::collections::BTreeSet;

struct DinnerPlates {
    v: Vec<Vec<i32>>,
    c: usize,
    e: BTreeSet<usize>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DinnerPlates {

    fn new(capacity: i32) -> Self {
        DinnerPlates {
            v: Vec::new(),
            c: capacity as usize,
            e: BTreeSet::new(),
        }
    }
    
    fn push(&mut self, val: i32) {
        if let Some(&i) = self.e.iter().next() {
            self.v[i].push(val);
            if self.v[i].len() == self.c as usize {
                self.e.remove(&i);
            }
        } else {
            if self.c > 1 {
                self.e.insert(self.v.len());
            }
            self.v.push(vec![ val ]);
        }
    }
    
    fn pop(&mut self) -> i32 {
        while let Some(ref s) = self.v.last() {
            if s.is_empty() {
                self.v.pop();
                self.e.remove(&self.v.len());
            } else {
                break;
            }
        }
        self.pop_at_stack(self.v.len() as i32 - 1)
    }
    
    fn pop_at_stack(&mut self, index: i32) -> i32 {
        if index >= 0 && (index as usize) < self.v.len() {
            if let Some(r) = self.v[index as usize].pop() {
                self.e.insert(index as usize);
                return r;
            }
        }
        -1
    }
}

/**
 * Your DinnerPlates object will be instantiated and called as such:
 * let obj = DinnerPlates::new(capacity);
 * obj.push(val);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.pop_at_stack(index);
 */