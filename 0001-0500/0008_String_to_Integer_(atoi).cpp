class Solution {
public:
    int myAtoi(string s) {
        static int MAX = 2147483647, MIN = -2147483648;
        int result = 0, sign = 1;
        size_t i = 0, n = s.size();
        // Leading space
        while (i < n && s[i] == ' ')
            i++;
        // Sign
        if (i < n && (s[i] == '-' || s[i] == '+')) {
            if (s[i] == '-')
                sign = -1;
            i++;
        }
        // Body
        while (i < n && isdigit(s[i])) {
            int digit = s[i] - '0';
            if (result > (MAX - digit) / 10)
                return sign > 0 ? MAX : MIN;
            result = result * 10 + digit;
            i++;
        }
        return sign * result;
    }
};