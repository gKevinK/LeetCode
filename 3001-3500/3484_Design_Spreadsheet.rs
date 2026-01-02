struct Spreadsheet {
    data: Vec<Vec<i32>>
}

fn parse_int(bytes: &[u8]) -> i32 {
    let mut num = 0;
    for &b in bytes {
        num = num * 10 + (b - b'0') as i32;
    }
    num
}

fn parse_cell(cell: &[u8]) -> (usize, usize) {
    let col = (cell[0] - b'A') as usize;
    let row = parse_int(&cell[1..]) as usize - 1;
    (col, row)
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Spreadsheet {

    fn new(rows: i32) -> Self {
        Spreadsheet {
            data: vec![vec![0; rows as usize]; 26]
        }
    }
    
    fn set_cell(&mut self, cell: String, value: i32) {
        let c = parse_cell(cell.as_bytes());
        self.data[c.0][c.1] = value;
    }
    
    fn reset_cell(&mut self, cell: String) {
        let c = parse_cell(cell.as_bytes());
        self.data[c.0][c.1] = 0;
    }
    
    fn get_value(&self, formula: String) -> i32 {
        let f = formula.as_bytes();
        let iplus = f.iter().position(|&x| x == b'+').unwrap();
        let a = if b'A' <= f[1] && f[1] <= b'Z' {
            let c = parse_cell(&f[1..iplus]);
            self.data[c.0][c.1]
        } else {
            parse_int(&f[1..iplus])
        };
        let b = if b'A' <= f[iplus + 1] && f[iplus + 1] <= b'Z' {
            let c = parse_cell(&f[iplus + 1..]);
            self.data[c.0][c.1]
        } else {
            parse_int(&f[iplus + 1..])
        };
        a + b
    }
}

/**
 * Your Spreadsheet object will be instantiated and called as such:
 * let obj = Spreadsheet::new(rows);
 * obj.set_cell(cell, value);
 * obj.reset_cell(cell);
 * let ret_3: i32 = obj.get_value(formula);
 */