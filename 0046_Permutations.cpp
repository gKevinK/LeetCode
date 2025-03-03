class Solution {
private:
    void pm(vector<int>& nums, int t, vector<vector<int>>& res) {
        if (t < nums.size() - 1) {
            for (int i = t; i < nums.size(); i++) {
                swap(nums[t], nums[i]);
                pm(nums, t + 1, res);
                swap(nums[t], nums[i]);
            }
        } else {
            res.push_back(nums);
        }
    }
public:
    vector<vector<int>> permute(vector<int>& nums) {
        vector<vector<int>> res;
        pm(nums, 0, res);
        return res;
    }
};