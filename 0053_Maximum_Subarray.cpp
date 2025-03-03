class Solution {
public:
    int maxSubArray(vector<int>& nums) {
        int sum = 0, res = nums[0];
        for (int& n : nums) {
            sum += n;
            res = max(res, sum);
            sum = max(sum, 0);
        }
        return res;
    }
};