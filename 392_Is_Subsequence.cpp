class Solution {
public:
    bool isSubsequence(string s, string t) {
        int is = 0, it = 0;
        for (; is < s.size() && it < t.size(); it++) {
            if (s[is] == t[it]) is++;
        }
        return is == s.size();
    }
};