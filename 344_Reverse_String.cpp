class Solution {
public:
    string reverseString(string s) {
        int l = s.length();
        for (int i = 0; i < l / 2; ++i) {
            swap(s[i], s[l-i-1]);
        }
        return s;
    }
};