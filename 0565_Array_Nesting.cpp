class Solution {
public:
    int arrayNesting(vector<int>& nums) {
        int n = nums.size(), r = 0;
        vector<bool> v(n, false);
        for (int i = 0; i < n; i++) {
            if (v[i]) continue;
            int t = i, l = 0;
            while (!v[t]) {
                v[t] = true;
                t = nums[t];
                l++;
            }
            r = max(r, l);
        }
        return r;
    }
};