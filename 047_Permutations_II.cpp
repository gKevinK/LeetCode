class Solution {
public:
    vector<vector<int>> permuteUnique(vector<int>& nums) {
        sort(nums.begin(), nums.end());
        vector<vector<int>> res = { nums };
        int n = nums.size();
        while (true) {
            int i = n - 2;
            while (i >= 0 && nums[i] >= nums[i + 1]) i--;
            if (i < 0) break;
            int j = i + 2;
            while (j < n && nums[j] > nums[i]) j++;
            swap(nums[i], nums[j - 1]);
            for (j = 1; i + j < n - j; j++)
                swap(nums[i + j], nums[n - j]);
            res.push_back(nums);
        }
        return res;
    }
};