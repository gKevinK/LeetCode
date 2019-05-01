class Solution {
public:
    int findNumberOfLIS(vector<int>& nums) {
        int len = 0, count = 0, n = nums.size();
        vector<pair<int, int>> dp(n, { 0, 1 });
        for (int i = 0; i < n; ++i) {
            for (int j = 0; j < i; ++j) {
                if (nums[i] > nums[j]) {
                    if (dp[i].first < dp[j].first)
                        dp[i] = dp[j];
                    else if (dp[i].first == dp[j].first)
                        dp[i].second += dp[j].second;
                }
            }
            ++dp[i].first;
        }
        pair<int, int> r = { 0, 0 };
        for (auto & p : dp)
            if (p.first > r.first)
                r = p;
            else if (p.first == r.first)
                r.second += p.second;
        return r.second;
    }
};