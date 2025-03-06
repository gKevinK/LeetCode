struct SnapshotArray {
    snap: i32,
    arr: Vec<Vec<(i32, i32)>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SnapshotArray {

    fn new(length: i32) -> Self {
        SnapshotArray {
            snap: 0,
            arr: vec![vec![(0, 0)]; length as usize],
        }
    }
    
    fn set(&mut self, index: i32, val: i32) {
        if self.snap == self.arr[index as usize].last().unwrap().0 {
            self.arr[index as usize].pop();
        }
        self.arr[index as usize].push((self.snap, val));
    }
    
    fn snap(&mut self) -> i32 {
        self.snap += 1;
        self.snap - 1
    }
    
    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let v = &self.arr[index as usize];
        let mut lo = 0;
        let mut hi = v.len();
        while lo + 1 < hi {
            let mi = (hi - lo) / 2 + lo;
            if v[mi].0 == snap_id {
                lo = mi;
                break;
            }
            if v[mi].0 < snap_id {
                lo = mi;
            } else {
                hi = mi;
            }
        }
        v[lo].1
    }
}

/**
 * Your SnapshotArray object will be instantiated and called as such:
 * let obj = SnapshotArray::new(length);
 * obj.set(index, val);
 * let ret_2: i32 = obj.snap();
 * let ret_3: i32 = obj.get(index, snap_id);
 */