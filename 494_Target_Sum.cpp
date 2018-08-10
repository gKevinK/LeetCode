class Solution {
public:
    int findTargetSumWays(vector<int>& nums, int S) {
        int sum = 0;
        for (int & i : nums)
            sum += i;
        if (S > sum) return 0;
        vector<int> dp(sum + S + 1, 0);
        dp[0] = 1;
        for (int & i : nums) {
            int d = i * 2;
            for (int j = sum + S; j >= d; j--)
                dp[j] = dp[j] + dp[j - d];
        }
        return dp[S + sum];
    }
};