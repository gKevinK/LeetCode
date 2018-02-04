class Solution {
public:
    vector<vector<int>> threeSum(vector<int>& nums) {
        vector<vector<int>> v;
        int a, b, c;
        sort(nums.begin(), nums.end());
        for (a = 0; a + 2 < nums.size(); ++a) {
            if (a >= 1 && nums[a] == nums[a-1]) continue;
            b = a + 1; c = nums.size() - 1;
            while (b < c) {
                if (nums[a] + nums[b] + nums[c] == 0) {
                    vector<int> t { nums[a], nums[b], nums[c] };
                    v.emplace_back(t);
                    while (b < c && nums[b] == nums[b+1]) ++b;
                    while (b < c && nums[c] == nums[c-1]) --c;
                    ++b; --c;
                } else if (nums[a] + nums[b] + nums[c] > 0) {
                    --c;
                } else {
                    ++b;
                }
            }
        }
        return v;
    }
};