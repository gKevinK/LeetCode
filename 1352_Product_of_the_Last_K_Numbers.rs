struct ProductOfNumbers {
    v: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {

    fn new() -> Self {
        Self {
            v: vec![ 1 ],
        }
    }
    
    fn add(&mut self, num: i32) {
        if num == 0 {
            self.v = vec![ 1 ];
        } else {
            self.v.push(self.v.last().unwrap() * num);
        }
    }
    
    fn get_product(&self, k: i32) -> i32 {
        let l = self.v.len();
        if (k as usize) < l {
            self.v[l - 1] / self.v[l - 1 - k as usize]
        } else {
            0
        }
    }
}

/**
 * Your ProductOfNumbers object will be instantiated and called as such:
 * let obj = ProductOfNumbers::new();
 * obj.add(num);
 * let ret_2: i32 = obj.get_product(k);
 */