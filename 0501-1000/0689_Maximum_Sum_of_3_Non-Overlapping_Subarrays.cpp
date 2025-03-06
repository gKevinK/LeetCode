class Solution {
public:
    vector<int> maxSumOfThreeSubarrays(vector<int>& nums, int k) {
        int n = nums.size();
        vector<int> sum(n + 1);
        vector<pair<int, vector<int>>> dp(n - 3 * k + 2, { 0, {} });
        for (int i = 0; i < n; ++i)
            sum[i + 1] = sum[i] + nums[i];
        for (int r = 1; r <= 3; ++r) {
            for (int i = 1; i <= n - 3 * k + 1; ++i) {
                int s = sum[i + r * k - 1] - sum[i + (r - 1) * k - 1];
                if (dp[i - 1].first >= dp[i].first + s)
                    dp[i] = dp[i - 1];
                else {
                    dp[i].first += s;
                    dp[i].second.push_back(i + (r - 1) * k - 1);
                }
            }
        }
        return dp.back().second;
    }
};