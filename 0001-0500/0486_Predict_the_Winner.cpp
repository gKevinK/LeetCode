class Solution {
    vector<vector<int>> dp;
    int n;
    
    int f(vector<int>& nums, int lo, int hi) {
        if (dp[lo][hi] == INT_MIN) {
            if (lo == hi)
                dp[lo][hi] = nums[lo];
            else {
                dp[lo][hi] = max(nums[lo] - f(nums, lo + 1, hi), nums[hi] - f(nums, lo, hi - 1));
            }
        }
        return dp[lo][hi];
    }
    
public:
    bool PredictTheWinner(vector<int>& nums) {
        n = nums.size();
        dp = vector<vector<int>>(n, vector<int>(n, INT_MIN));
        return f(nums, 0, n - 1) >= 0;
    }
};