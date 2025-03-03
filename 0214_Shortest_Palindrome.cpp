class Solution {
public:
    string shortestPalindrome(string s) {
        if (s.size() <= 1) return s;
        int n = s.size();
        for (int i = (n - 1) / 2, j = i + 1; j >= 0; i--, j = i + 1) {
            while (i - 1 >= 0 && s[i - 1] == s[i]) i--;
            while (j < n && s[i] == s[j]) j++;
            bool p = true;
            for (int k = 0; i - k >= 0; k++) {
                if (s[i - k] == s[j - 1 + k]) continue;
                p = false;
                break;
            }
            if (p) {
                string t = s.substr(i + j, n - i - j);
                return string(t.rbegin(), t.rend()) + s;
            }
        }
    }
};