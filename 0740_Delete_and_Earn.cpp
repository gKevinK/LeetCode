class Solution {
public:
    int deleteAndEarn(vector<int>& nums) {
        if (nums.empty()) return 0;
        sort(nums.begin(), nums.end());
        int n = nums.size(), i1 = 0, i2 = nums[0], t1, t2;
        for (int i = 1; i < n; i++) {
            if (nums[i] == nums[i - 1]) {
                i2 = i2 + nums[i];
                continue;
            }
            t1 = max(i1, i2);
            t2 = (nums[i] == nums[i - 1] + 1 ? i1 : t1) + nums[i];
            i1 = t1;
            i2 = t2;
        }
        return max(i1, i2);
    }
};