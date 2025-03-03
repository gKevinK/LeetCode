class Solution {
public:
    int combinationSum4(vector<int>& nums, int target) {
        vector<int> t(target + 1, 0);
        t[0] = 1;
        for (int i = 1; i <= target; ++i) {
            for (int& j : nums) {
                if (i - j >= 0) {
                    t[i] += t[i - j];
                }
            }
        }
        return t[target];
    }
};