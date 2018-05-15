class Solution {
public:
    int countBinarySubstrings(string s) {
        int l = 0, m = 0, r = 0, n = 0;
        while (m < s.size() && s[m] == s[l]) m++;
        r = m;
        while (r < s.size() && s[r] == s[m]) r++;
        n += min(m - l, r - m);
        while (r < s.size()) {
            l = m;
            m = r;
            while (r < s.size() && s[r] == s[m]) r++;
            n += min(m - l, r - m);
        }
        return n;
    }
};