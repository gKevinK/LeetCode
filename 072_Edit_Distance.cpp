class Solution {
public:
    int minDistance(string word1, string word2) {
        int n1 = word1.size(), n2 = word2.size();
        vector<vector<int>> m(n1 + 1, vector<int>(n2 + 1, 0));
        for (int i = 1; i <= n1; i++)
            m[i][0] = i;
        for (int j = 1; j <= n2; j++)
            m[0][j] = j;
        for (int i = 1; i <= n1; i++) {
            for (int j = 1; j <= n2; j++) {
                m[i][j] = min(m[i - 1][j], m[i][j - 1]) + 1;
                m[i][j] = min(m[i][j], m[i - 1][j - 1] + 1 - (word1[i - 1] == word2[j - 1]));
            }
        }
        return m[n1][n2];
    }
};