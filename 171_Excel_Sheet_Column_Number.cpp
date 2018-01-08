class Solution {
public:
    int titleToNumber(string s) {
        int r = 0;
        for (char c : s) {
            r = r * 26 + 1 + (c - 'A');
        }
        return r;
    }
};