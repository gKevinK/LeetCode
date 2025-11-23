class Solution {
public:
    bool isNumber(string s) {
        size_t i = 0, n = s.size();

        if (i == n) return false;
        if (s[i] == '+' || s[i] == '-') i++;

        if (i == n) return false;
        int digit = 0, dot = 0;
        while (i < n && (('0' <= s[i] && s[i] <= '9') || s[i] == '.')) {
            if (s[i] == '.') dot++;
            else digit++;
            i++;
        }
        if (dot > 1 || digit == 0) return false;
        if (i == n) return true;

        if (s[i] == 'e' || s[i] == 'E') {
            i++;
            if (i == n) return false;
            if (s[i] == '+' || s[i] == '-') i++;
            int e = 0;
            while (i < n && '0' <= s[i] && s[i] <= '9') {
                e++;
                i++;
            }
            if (e == 0) return false;
        }
        return i == n;
    }
};