class Solution {
public:
    vector<int> findDuplicates(vector<int>& nums) {
        vector<int> res;
        for (int i = 0; i < nums.size(); i++) {
            if (nums[i] == i + 1 || nums[i] == 0)
                continue;
            else if (nums[nums[i] - 1] == nums[i]) {
                res.push_back(nums[i]);
                nums[i] = 0;
            } else if (nums[i] != i + 1) {
                swap(nums[nums[i] - 1], nums[i]);
                i--;
            }
        }
        return res;
    }
};