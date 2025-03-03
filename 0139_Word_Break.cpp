class Solution {
private:
    bool sub(string& s, unordered_set<string>& dict, vector<int>& v, int r) {
        if (r == s.size()) return true;
        if (v[r] == 2) return true;
        else if (v[r] == 1) return false;
        string s1 = "";
        bool res = false;
        for (int i = r; i < s.size(); ++i) {
            s1.push_back(s[i]);
            auto iter = dict.find(s1);
            if (iter != dict.end()) {
                res = sub(s, dict, v, i + 1);
                if (res == true) {
                    v[r] = 2;
                    return true;
                }
            }
        }
        v[r] = 1;
        return false;
    }

public:
    bool wordBreak(string s, unordered_set<string>& wordDict) {
        vector<int> v = { 0 };
        v.resize(s.size() + 1);
        return sub(s, wordDict, v, 0);
    }
};