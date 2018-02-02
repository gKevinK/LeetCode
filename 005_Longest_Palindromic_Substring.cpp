class Solution {
public:
    string longestPalindrome(string s) {
        if (s.size() <= 1) return s;
        int i, j, k, maxr = 0, maxl = 0;
        for (i = 0, j = 1; i < s.size(); i = k, j = i + 1) {
            while (j < s.size() && s[i] == s[j]) ++j;
            k = j;
            while (i > 0 && j < s.size() && s[i-1] == s[j]) {
                --i; ++j;
            }
            if (j - i > maxl) {
                maxl = j - i; maxr = i;
            }
        }
        return s.substr(maxr, maxl);
    }
};