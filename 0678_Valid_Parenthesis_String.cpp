class Solution {
public:
    bool checkValidString(string s) {
        int s1 = 0, s2 = 0;
        for (char c : s) {
            if (c == '(')
                ++s1, ++s2;
            else if (c == ')')
                --s1, --s2;
            else
                --s1, ++s2;
            s1 = max(s1, 0);
            if (s1 > s2) return false;
        }
        return s1 <= 0 && s2 >= 0;
    }
};