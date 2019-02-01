class Solution {
    bool subseq(const string & a, const string & b) {
        int j = 0;
        for (int i = 0; i < a.size() && j < b.size(); i++)
            if (a[i] == b[j])
                j++;
        return j == b.size();
    }
public:
    int findLUSlength(vector<string>& strs) {
        unordered_map<string, int> m;
        for (auto & str : strs)
            m[str]++;
        vector<pair<string, int>> v(m.begin(), m.end());
        sort(v.begin(), v.end(), [](auto & a, auto & b) {
            return a.first.size() != b.first.size() ? a.first.size() > b.first.size() : a < b;
        });
        unordered_set<string> s;
        for (auto & p : v) {
            cout << p.first << endl;
            if (p.second == 1) {
                bool flag = true;
                for (auto & w : s)
                    if (subseq(w, p.first))
                        flag = false;
                if (flag) return p.first.size();
            }
            s.insert(p.first);
        }
        return -1;
    }
};