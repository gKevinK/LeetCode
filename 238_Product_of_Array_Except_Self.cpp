class Solution {
public:
    vector<int> productExceptSelf(vector<int>& nums) {
        int n = nums.size();
        vector<int> out(n, 0);
        out[0] = 1;
        for (int i = 1; i < n; ++i) {
            out[i] = nums[i-1] * out[i-1];
        }
        int c = nums[n-1];
        for (int i = n - 2; i >= 0; --i) {
            out[i] *= c;
            c *= nums[i];
        }
        return out;
    }
};