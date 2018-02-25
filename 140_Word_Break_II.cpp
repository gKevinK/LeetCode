class Solution {
    void f(string & s, vector<vector<int>> & vvi, vector<vector<string>> & vv, int c) {
        for (int & i : vvi[c]) {
            if (i == -1)
                vv[c].push_back(s.substr(0, c + 1));
            else {
                if (vv[i].empty() && !vvi[i].empty())
                    f(s, vvi, vv, i);
                for (string & vs : vv[i])
                    vv[c].push_back(vs + " " + s.substr(i + 1, c - i));
            }
        }
    }
public:
    vector<string> wordBreak(string s, vector<string>& wordDict) {
        if (s.size() == 0) return vector<string>();
        vector<vector<int>> vvi(s.size(), vector<int>());
        vector<vector<string>> vv(s.size(), vector<string>());
        unordered_set<string> set(wordDict.begin(), wordDict.end());
        for (int i = 0; i < s.size(); i++) {
            string ss = s.substr(0, i + 1);
            if (set.find(ss) != set.end())
                vvi[i].push_back(-1);
            for (int j = 0; j < i; j++) {
                if (vvi[j].empty()) continue;
                string ss = s.substr(j + 1, i - j);
                if (set.find(ss) != set.end())
                    vvi[i].push_back(j);
            }
        }
        f(s, vvi, vv, s.size()-1);
        return vv.back();
    }
};