class Solution {
public:
    vector<vector<int>> fourSum(vector<int>& nums, int target) {
        sort(nums.begin(), nums.end());
        vector<vector<int>> result;
        size_t n = nums.size();
        for (size_t i = 0; i + 3 < n; i++) {
            for (size_t j = i + 1; j + 2 < n; j++) {
                int64_t t = (int64_t)target - (int64_t)nums[i] - (int64_t)nums[j];
                size_t k = j + 1, l = n - 1;
                while (k < l) {
                    if (nums[k] + nums[l] < t) {
                        k++;
                    } else if (nums[k] + nums[l] > t) {
                        l--;
                    } else {
                        result.push_back({nums[i], nums[j], nums[k], nums[l]});
                        while (k + 1 < l && nums[k] == nums[k + 1]) k++;
                        k++;
                        while (k + 1 < l && nums[l - 1] == nums[l]) l--;
                        l--;
                    }
                }
                while (j + 1 < n && nums[j] == nums[j + 1]) j++;
            }
            while (i + 1 < n && nums[i] == nums[i + 1]) i++;
        }
        return result;
    }
};