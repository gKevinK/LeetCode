class Solution {
public:
    int uniqueLetterString(string S) {
        int r = 0, MOD = 1'000'000'007, n = S.size();
        for (char c = 'A'; c <= 'Z'; ++c) {
            int l = -1, m = 0, h = 0;
            while (m < n && S[m] != c) ++m;
            h = m + 1;
            while (h < n && S[h] != c) ++h;
            while (m < n) {
                r = (r + (h - m) * (m - l)) % MOD;
                l = m;
                m = h;
                ++h;
                while (h < n && S[h] != c) ++h;
            }
        }
        return r;
    }
};