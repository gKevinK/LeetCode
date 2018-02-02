class Solution {
public:
    int lengthOfLongestSubstring(string s) {
        int r[128] = { 0 };
        for (int i = 0; i < 128; ++i) {
            r[i] = -1;
        }
        int left = 0, len = 0;
        for (int i = 0; i < s.length(); ++i) {
            for (; r[s[i]] != -1; ++left) {
                r[s[left]] = -1;
            }
            r[s[i]] = i;
            if (len < i - left + 1) {
                len = i - left + 1;
            }
        }
        return len;
    }
};