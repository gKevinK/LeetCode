impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut r = vec![];
        let un = n as usize;
        r.reserve(un * 2);
        for i in 0..un {
            r.push(nums[i]);
            r.push(nums[i + un]);
        }
        r
    }
}