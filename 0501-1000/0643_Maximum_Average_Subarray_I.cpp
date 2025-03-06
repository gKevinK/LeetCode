class Solution {
public:
    double findMaxAverage(vector<int>& nums, int k) {
        int s = 0, r = 0;
        for (int i = 0; i < k && i < nums.size(); i++)
            s += nums[i];
        r = s;
        for (int i = 1; i + k <= nums.size(); i++) {
            s = s - nums[i-1] + nums[i+k-1];
            r = max(r, s);
        }
        return r * 1.0 / k;
    }
};