class Solution {
public:
    string removeDuplicateLetters(string s) {
        vector<int> v1(26, 0), v2;
        for (char & c : s)
            v1[c - 'a']++;
        string r;
        int p = 0;
        while (p < s.size()) {
            v2 = v1;
            for (int i = p; i < s.size(); i++) {
                if (v1[s[i] - 'a'] > 0 && (v1[s[p] - 'a'] < 0 || s[i] < s[p])) {
                    p = i; v2 = v1;
                }
                if (--v1[s[i] - 'a'] == 0) break;
            }
            v1 = v2;
            if (v1[s[p] - 'a'] <= 0) break;
            r += s[p];
            v1[s[p++] - 'a'] = -1;
        }
        return r;
    }
};