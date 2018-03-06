class Solution {
public:
    int numMatchingSubseq(string S, vector<string>& words) {
        vector<pair<int, int>> vs[26];
        for (int i = 0; i < words.size(); i++) {
            vs[words[i][0] - 'a'].push_back(make_pair(i, 0));
        }
        int n = 0;
        for (char & c : S) {
            auto tmp = vs[c - 'a'];
            vs[c - 'a'].clear();
            for (auto & t : tmp) {
                if (t.second + 1 == words[t.first].size()) {
                    n++;
                    continue;
                }
                vs[words[t.first][t.second + 1] - 'a'].push_back(make_pair(t.first, t.second + 1));
            }
        }
        return n;
    }
};