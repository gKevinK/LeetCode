class Solution {
public:
    int longestPalindrome(string s) {
        vector<int> v(52, 0);
        for (char c : s) {
            int i = (c <= 'Z' ? c - 'A' : c - 'a' + 26);
            v[i]++;
        }
        int odd = 0;
        for (int i : v) {
            odd += i % 2;
        }
        return s.size() - odd + (odd > 0);
    }
};