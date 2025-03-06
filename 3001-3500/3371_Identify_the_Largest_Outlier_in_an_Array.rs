impl Solution {
    pub fn get_largest_outlier(nums: Vec<i32>) -> i32 {
        let sum_total: i32 = nums.iter().sum();
        let mut freq = std::collections::HashMap::new();
        for &num in &nums {
            *freq.entry(num).or_insert(0) += 1;
        }
        
        let mut max_outlier = i32::MIN;
        
        for &candidate in &nums {
            let required_sum = sum_total - candidate;
            if required_sum % 2 != 0 {
                continue;
            }
            let sum_candidate = required_sum / 2;
            
            if !freq.contains_key(&sum_candidate) {
                continue;
            }
            
            let valid = if sum_candidate != candidate {
                true
            } else {
                // Ensure there are at least two occurrences of the sum_candidate
                freq.get(&sum_candidate).map_or(false, |&count| count >= 2)
            };
            
            if valid {
                if candidate > max_outlier {
                    max_outlier = candidate;
                }
            }
        }
        
        max_outlier
    }
}