class Solution {
public:
    string reverseStr(string s, int k) {
        for (int i = 0, n = s.size(); i < n; i += 2 * k) {
            for (int j1 = i, j2 = min(i + k, n) - 1; j1 < j2; j1++, j2--)
                swap(s[j1], s[j2]);
        }
        return s;
    }
};