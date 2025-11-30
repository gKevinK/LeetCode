impl Solution {
    pub fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool {
        let c1 = coordinate1.as_bytes();
        let x1 = (c1[0] - b'a' + c1[1] - b'1') as i32 % 2;
        let c2 = coordinate2.as_bytes();
        let x2 = (c2[0] - b'a' + c2[1] - b'1') as i32 % 2;
        x1 == x2
    }
}