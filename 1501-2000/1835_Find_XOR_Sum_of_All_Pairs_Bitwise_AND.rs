impl Solution {
    pub fn get_xor_sum(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut a1 = 0;
        let mut a2 = 0;
        for i in arr1 {
            a1 ^= i;
        }
        for i in arr2 {
            a2 ^= i;
        }
        a1 & a2
    }
}