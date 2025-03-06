class Solution {
public:
    int findRotateSteps(string ring, string key) {
        int m = key.size(), n = ring.size();
        auto dp = vector<vector<int>>(m, vector<int>(n, INT_MAX));
        unordered_map<char, unordered_set<int>> mp;
        for (int i = 0; i < n; i++)
            dp[0][i] = min(i, n - i);
        for (int i = 0; i < n; i++)
            mp[ring[i]].insert(i);
        for (int i = 1; i < m; i++) {
            for (int j : mp[key[i]]) {
                for (int k : mp[key[i - 1]]) {
                    int diff = abs(j - k);
                    diff = min(diff, n - diff);
                    dp[i][j] = min(dp[i][j], dp[i - 1][k] + diff);
                }
            }
        }
        return *min_element(dp[m - 1].begin(), dp[m - 1].end()) + m;
    }
};