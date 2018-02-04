class Solution {
public:
    int threeSumClosest(vector<int>& nums, int target) {
        vector<int> v { 0, 0, 0 };
        int a, b, c, z = INT_MAX;
        sort(nums.begin(), nums.end());
        for (a = 0; a + 2 < nums.size(); ++a) {
            if (a >= 1 && nums[a] == nums[a-1]) continue;
            b = a + 1; c = nums.size() - 1;
            while (b < c) {
                if (abs(nums[a] + nums[b] + nums[c] - target) < z) {
                    v[0] = nums[a];
                    v[1] = nums[b];
                    v[2] = nums[c];
                    z = abs(nums[a] + nums[b] + nums[c] - target);
                }
                nums[a] + nums[b] + nums[c] - target > 0 ? --c : ++b;
            }
        }
        return v[0] + v[1] + v[2];
    }
};