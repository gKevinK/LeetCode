impl Solution {
    pub fn reinitialize_permutation(n: i32) -> i32 {
        let mut m = 0;
        let mut arr = vec![0; n as usize];
        for i in 0..n as usize {
            arr[i] = i;
        }
        loop {
            m += 1;
            let mut arr_2 = vec![0; n as usize];
            for i in 0..n as usize {
                if i % 2 == 0 {
                    arr_2[i] = arr[i / 2];
                } else {
                    arr_2[i] = arr[n as usize / 2 + (i - 1) / 2];
                }
            }
            arr = arr_2;
            let mut f = true;
            for i in 0..n as usize {
                if arr[i] != i {
                    f = false;
                    break;
                }
            }
            if f {
                return m;
            }
        }
    }
}