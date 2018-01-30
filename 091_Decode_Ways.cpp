class Solution {
public:
    int numDecodings(string s) {
        if (s.size() == 0) return 0;
        int a1 = 1, a2 = 1;
        for (int i = 0; i < s.size(); i++) {
            int t = 0;
            if (s[i] > '0') t += a2;
            if (i > 0 && (s[i-1] == '1' || (s[i-1] == '2' && s[i] <= '6'))) t += a1;
            a1 = a2;
            a2 = t;
        }
        return a2;
    }
};