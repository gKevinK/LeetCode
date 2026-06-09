impl Solution {
    pub fn can_make_equal(nums: Vec<i32>, k: i32) -> bool {
        let mut pos_op = 0;
        let mut neg_op = 0;
        let mut pos_mul = 1;
        let mut neg_mul = 1;
        for num in nums {
            if num * pos_mul > 0 {
                pos_mul = 1;
            } else {
                pos_op += 1;
                pos_mul = -1;
            }
            if num * neg_mul < 0 {
                neg_mul = 1;
            } else {
                neg_op += 1;
                neg_mul = -1;
            }
        }
        (pos_op <= k && pos_mul == 1) || (neg_op <= k && neg_mul == 1)
    }
}