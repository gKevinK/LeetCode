impl Solution {
    pub fn min_jumps(nums: Vec<i32>) -> i32 {
        use std::sync::LazyLock;
        const LIMIT: usize = 1000000;
        static FACTOR: LazyLock<[i32; LIMIT + 1]> = LazyLock::new(|| {
            let mut f = [0; LIMIT + 1];
            let mut i = 2;
            while i <= LIMIT {
                if f[i] == 0 {
                    f[i] = i as i32;
                    let mut j = i * i;
                    while j <= LIMIT {
                        if f[j] == 0 {
                            f[j] = i as i32;
                        }
                        j += i;
                    }
                }
                i += 1;
            }
            f
        });

        let mut map = std::collections::HashMap::new();
        let n = nums.len();
        for i in 0..n {
            let mut num = nums[i];
            while num > 1 {
                let f = FACTOR[num as usize];
                map.entry(f).or_insert(Vec::new()).push(i);
                while num % f == 0 {
                    num /= f;
                }
            }
        }
        let mut visit = vec![false; n];
        visit[0] = true;
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((0, 0));
        while let Some((jump, i)) = queue.pop_front() {
            if i + 1 == n {
                return jump;
            }
            if i > 0 && visit[i - 1] == false {
                visit[i - 1] = true;
                queue.push_back((jump + 1, i - 1));
            }
            if i + 1 < n && visit[i + 1] == false {
                visit[i + 1] = true;
                queue.push_back((jump + 1, i + 1));
            }
            if FACTOR[nums[i] as usize] == nums[i] {
                if let Some(mut next) = map.get_mut(&nums[i]) {
                    for j in next.drain(..) {
                        if visit[j] == false {
                            if j + 1 == n {
                                return jump + 1;
                            }
                            visit[j] = true;
                            queue.push_back((jump + 1, j));
                        }
                    }
                }
            }
        }
        0
    }

    fn is_prime(n: i32) -> bool {
        if n <= 1 {
            return false;
        }
        if n == 2 || n == 3 {
            return true;
        }
        if n % 2 == 0 || n % 3 == 0 {
            return false;
        }
        let mut i = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }
}