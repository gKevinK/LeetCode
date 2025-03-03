class Solution {
public:
    bool isMatch(string s, string p) {
        string m = "^" + p + "$";
        return regex_match(s, regex(m));
    }
};