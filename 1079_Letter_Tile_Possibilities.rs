impl Solution {
    fn f(arr: &mut Vec<i32>) -> i32 {
        let mut sum = 0;
        for i in 0..26 {
            if arr[i] == 0 {
                continue;
            }
            arr[i] -= 1;
            sum += 1;
            sum += Self::f(arr);
            arr[i] += 1;
        }
        sum
    }
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut arr = vec![0; 26];
        for ch in tiles.chars() {
            arr[(ch as u32 - 'A' as u32) as usize] += 1;
        }
        Self::f(&mut arr)
    }
}