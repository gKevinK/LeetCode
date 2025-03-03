class Solution {
public:
    int longestSubstring(string s, int k) {
        int res = 0;
        for (int h = 1; h <= 26; h++) {
            vector<int> count(26, 0);
            int i = 0, j = 0, lz = 0, lek = 0;
            while (j < s.size()) {
                if (lz <= h) {
                    int & c = count[s[j] - 'a'];
                    c++;
                    if (c == 1) lz++;
                    if (c == k) lek++;
                    j++;
                } else {
                    int & c = count[s[i] - 'a'];
                    if (c == k) lek--;
                    if (c == 1) lz--;
                    c--;
                    i++;
                }
                if (lz == h && lz == lek) {
                    res = max(res, j - i);
                }
            }
        }
        return res;
    }
};