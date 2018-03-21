class Solution {
public:
    vector<vector<int>> subsets(vector<int>& nums) {
        int n = nums.size();
        vector<vector<int>> vec;
        for (long long t = 0; t < 1 << n; t++) {
            long long a = t;
            vector<int> v;
            for (int i = 0; a > 0 && i < n; i++) {
                if (a & 1) {
                    v.push_back(nums[i]);
                }
                a = a >> 1;
            }
            vec.push_back(v);
        }
        return vec;
    }
};