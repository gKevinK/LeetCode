struct Robot {
    h: i32,
    a: i32,
    b: i32,
    c: i32,
    m: i32,
    pos: i32,
    dir: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Robot {

    fn new(width: i32, height: i32) -> Self {
        Robot {
            h: height,
            a: width,
            b: width + height - 1,
            c: width * 2 + height - 2,
            m: width * 2 + height * 2 - 4,
            pos: 0,
            dir: 0,
        }
    }

    fn step(&mut self, num: i32) {
        if num > 0 {
            self.pos = (self.pos + num) % self.m;
            if self.pos == 0 {
                self.dir = 3;
            } else if self.pos < self.a {
                self.dir = 0;
            } else if self.pos < self.b {
                self.dir = 1;
            } else if self.pos < self.c {
                self.dir = 2;
            } else {
                self.dir = 3;
            }
        }
    }
    
    fn get_pos(&self) -> Vec<i32> {
        if self.pos == 0 {
            vec![0, 0]
        } else if self.pos < self.a {
            vec![self.pos, 0]
        } else if self.pos < self.b {
            vec![self.a - 1, self.pos - self.a + 1]
        } else if self.pos < self.c {
            vec![self.a - 2 - (self.pos - self.b), self.h - 1]
        } else {
            vec![0, self.h - 2 - (self.pos - self.c)]
        }
    }
    
    fn get_dir(&self) -> String {
        match self.dir {
            0 => "East",
            1 => "North",
            2 => "West",
            3 => "South",
            _ => "None",
        }.to_string()
    }
}

/**
 * Your Robot object will be instantiated and called as such:
 * let obj = Robot::new(width, height);
 * obj.step(num);
 * let ret_2: Vec<i32> = obj.get_pos();
 * let ret_3: String = obj.get_dir();
 */