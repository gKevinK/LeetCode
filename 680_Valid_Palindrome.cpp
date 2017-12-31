class Solution {
    bool valid(string & s, int i, int j) {
        while (i < j)
            if (s[i++] != s[j--])
                return false;
        return true;
    }
public:
    bool validPalindrome(string s) {
        for (int i = 0, j = s.size() - 1; i < j; i++, j--) {
            if (s[i] != s[j]) {
                return valid(s, i, j-1) || valid(s, i+1, j);
            }
        }
        return true;
    }
};