class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        int i, j;
        vector<int> r(2);
        for (i = 0; i < nums.size() - 1; i++) {
            for (j = i + 1; j < nums.size(); j++) {
                if (nums[i] + nums[j] == target) {
                    r[0] = i;
                    r[1] = j;
                    return r;
                }
            }
        }
        return r;
    }
};