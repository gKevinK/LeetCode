class Solution {
public:
    string findReplaceString(string S, vector<int>& indexes, vector<string>& sources, vector<string>& targets) {
        string r;
        map<int, int> m;
        for (int i = 0; i < indexes.size(); i++)
            m[indexes[i]] = i;
        for (int i = 0; i < S.size(); ) {
            if (m.find(i) != m.end() && S.substr(i, sources[m[i]].size()) == sources[m[i]]) {
                r += targets[m[i]];
                i += sources[m[i]].size();
            } else {
                r.push_back(S[i++]);
            }
        }
        return r;
    }
};