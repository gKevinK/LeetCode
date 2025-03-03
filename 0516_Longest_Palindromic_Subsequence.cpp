class Solution {
public:
    int longestPalindromeSubseq(string s) {
        int len = s.size();
        vector<int> v1(len, 1), v2(len, 1);
        for (int i = 1; i < len; i++) {
            for (int j = i - 1; j >= 0; j--) {
                if (s[i] == s[j])
                    v2[j] = max(v1[j], (j + 1 == i ? 0 : v1[j + 1]) + 2);
                else
                    v2[j] = max(v1[j], v2[j + 1]);
            }
            v1.swap(v2);
        }
        return v1.front();
    }
};