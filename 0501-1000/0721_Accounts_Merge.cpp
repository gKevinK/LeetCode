class Solution {
public:
    vector<vector<string>> accountsMerge(vector<vector<string>>& accounts) {
        unordered_map<string, unordered_map<string, string>> m;
        for (auto & a : accounts) {
            auto & mi = m[a[0]];
            if (mi.find(a[1]) == mi.end()) mi[a[1]] = a[1];
            string p1 = a[1];
            while (mi[p1] != p1) p1 = mi[p1];
            for (int i = 2; i < a.size(); ++i) {
                string p2 = a[i];
                while (mi.find(p2) != mi.end() && mi[p2] != p2) p2 = mi[p2];
                mi[p2] = p1;
            }
        }
        unordered_map<string, unordered_map<string, set<string>>> u;
        for (auto & p1 : m) {
            auto & mi = p1.second;
            for (auto & p2 : p1.second) {
                string p = p2.second;
                while (mi[p] != p) p = mi[p];
                u[p1.first][p].insert(p2.first);
            }
        }
        vector<vector<string>> v;
        for (auto & p1 : u) {
            for (auto & p2 : p1.second) {
                vector<string> a = { p1.first };
                for (auto & s : p2.second)
                    a.push_back(s);
                v.push_back(a);
            }
        }
        return v;
    }
};