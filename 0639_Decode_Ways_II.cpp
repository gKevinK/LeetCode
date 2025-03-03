class Solution {
public:
    int numDecodings(string s) {
        if (s.size() == 0) return 0;
        long long a1 = 1, a2 = 1;
        for (int i = 0; i < s.size(); i++) {
            long long t = 0;
            if (s[i] > '0') t += a2;
            else if (s[i] == '*') t += a2 * 9;
            if (i > 0 && (s[i-1] == '1' || (s[i-1] == '2' && s[i] <= '6') || s[i-1] == '*')) {
                if (s[i-1] == '1') t += s[i] == '*' ? a1 * 9 : a1;
                if (s[i-1] == '2') t += s[i] == '*' ? a1 * 6 : a1;
                if (s[i-1] == '*') t += s[i] == '*' ? a1 * 15 : (s[i] <= '6' ? a1 * 2 : a1);
            }
            a1 = a2;
            a2 = t % ((int)1e9 + 7);
        }
        return a2;
    }
};