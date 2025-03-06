impl Solution {
    fn parse(date: &String) -> (i32, i32, i32) {
        let mut state = 0;
        let mut r = [ 0, 0, 0 ];
        for c in date.chars() {
            if c == '-' {
                state += 1;
            } else {
                r[state as usize] *= 10;
                r[state as usize] += (c as i32 - '0' as i32);
            }
        }
        (r[0], r[1], r[2])
    }
    fn is_leap_year(year: i32) -> bool {
        if year % 400 == 0 { return true; }
        if year % 100 == 0 { return false; }
        year % 4 == 0
    }
    pub fn days_between_dates(date1: String, date2: String) -> i32 {
        let d1 = Self::parse(&date1);
        let d2 = Self::parse(&date2);
        let md = [ 0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31 ];
        let mut r = 0;
        let sy = d1.0.min(d2.0);
        for i in sy..d1.0 {
            r += if Self::is_leap_year(i) { 366 } else { 365 };
        }
        for i in 1..d1.1 {
            r += md[i as usize];
        }
        if Self::is_leap_year(d1.0) && d1.1 > 2 {
            r += 1;
        }
        r += d1.2;
        r = -r;
        for i in sy..d2.0 {
            r += if Self::is_leap_year(i) { 366 } else { 365 };
        }
        for i in 1..d2.1 {
            r += md[i as usize];
        }
        if Self::is_leap_year(d2.0) && d2.1 > 2 {
            r += 1;
        }
        r += d2.2;
        r.abs()
    }
}