class Solution {
public:
    int characterReplacement(string s, int k) {
        vector<int> count(26, 0);
        int n = s.size(), l = 0, len = 0, mc = 0;
        for (int i = 0; i < n; i++) {
            mc = max(mc, ++count[s[i] - 'A']);
            while (i - l + 1 - mc > k) {
                count[s[l] - 'A']--;
                l++;
            }
            len = max(len, i - l + 1);
        }
        return len;
    }
};