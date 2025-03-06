class Solution {
public:
    bool isIsomorphic(string s, string t) {
        vector<char> v1(256, ' '), v2(256, ' ');
        for (int i = 0; i < s.size(); i++) {
            if ((v1[s[i]] != ' ' && v1[s[i]] != t[i]) || (v2[t[i]] != ' ' && v2[t[i]] != s[i])) return false;
            v1[s[i]] = t[i];
            v2[t[i]] = s[i];
        }
        return true;
    }
};