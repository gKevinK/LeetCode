class Solution {
public:
    vector<vector<string>> findDuplicate(vector<string>& paths) {
        unordered_map<string, unordered_set<string>> m;
        for (string& path : paths) {
            int i = 0;
            while (path[i] != ' ') i++;
            string d = path.substr(0, i);
            while (i < path.size()) {
                int i2 = ++i;
                while (path[i] != '(') i++;
                string f = path.substr(i2, i - i2);
                i2 = ++i;
                while (path[i] != ')') i++;
                string c = path.substr(i2, i - i2);
                i++;
                m[c].insert(d + '/' + f);
            }
        }
        vector<vector<string>> res;
        for (auto & p : m)
            if (p.second.size() >= 2)
                res.emplace_back(vector<string>(p.second.begin(), p.second.end()));
        return res;
    }
};