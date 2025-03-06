class Solution {
public:
    int minDistance(string word1, string word2) {
        string & w1 = word1.size() < word2.size() ? word2 : word1;
        string & w2 = word1.size() < word2.size() ? word1 : word2;
        int l1 = w1.size(), l2 = w2.size();
        if (l2 == 0) return l1;
        vector<int> dp(l2 + 1), dp2(l2 + 1);
        for (int i = 0; i <= l1; i++) {
            for (int j = 0; j <= l2; j++) {
                if (i == 0 || j == 0)
                    dp2[j] = max(i, j);
                else if (w1[i - 1] == w2[j - 1])
                    dp2[j] = dp[j - 1];
                else
                    dp2[j] = min(dp[j], dp2[j - 1]) + 1;
            }
            dp.swap(dp2);
        }
        return dp[l2];
    }
};