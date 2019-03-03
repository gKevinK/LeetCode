class Solution {
public:
    int mincostTickets(vector<int>& days, vector<int>& costs) {
        vector<int> dp(366, INT_MAX);
        int curr = 0;
        for (int & d : days) {
            dp[d] = min(dp[d], curr + costs[0]);
            for (int i = d; i < d + 7 && i < 366; i++)
                dp[i] = min(dp[i], curr + costs[1]);
            for (int i = d; i < d + 30 && i < 366; i++)
                dp[i] = min(dp[i], curr + costs[2]);
            curr = dp[d];
        }
        return curr;
    }
};