class Solution {
public:
    vector<double> calcEquation(vector<pair<string, string>> equations, vector<double>& values, vector<pair<string, string>> queries) {
        int n = values.size();
        unordered_map<string, vector<pair<double, string>>> me;
        for (int i = 0; i < n; i++) {
            me[equations[i].first].push_back({ values[i], equations[i].second });
            me[equations[i].second].push_back({ 1.0 / values[i], equations[i].first });
        }
        vector<double> res;
        for (int i = 0; i < queries.size(); i++) {
            auto query = queries[i];
            if (me.find(query.first) == me.end() || me.find(query.second) == me.end()) {
                res.push_back(-1.0);
                continue;
            } else if (query.first == query.second) {
                res.push_back(1.0);
                continue;
            }
            deque<pair<double, string>> q;
            set<string> s;
            q.push_back({ 1.0, query.first });
            s.insert(query.first);
            while (!q.empty()) {
                auto p = q.front();
                q.pop_front();
                for (auto & e : me[p.second]) {
                    if (e.second == query.second) {
                        res.push_back(p.first * e.first);
                        q.clear();
                        break;
                    }
                    if (s.find(e.second) == s.end()) {
                        q.push_back({ p.first * e.first, e.second });
                        s.insert(e.second);
                    }
                }
            }
            if (res.size() <= i)
                res.push_back(-1.0);
        }
        return res;
    }
};