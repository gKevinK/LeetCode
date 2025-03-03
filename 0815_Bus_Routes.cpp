class Solution {
public:
    int numBusesToDestination(vector<vector<int>>& routes, int S, int T) {
        if (S == T)
            return 0;
        unordered_set<int> s1, s2;
        vector<bool> v(routes.size(), false);
        unordered_multimap<int, int> m;
        for (int i = 0; i < routes.size(); i++)
            for (int & j : routes[i])
                m.emplace(j, i);
        s1.insert(S);
        s2.insert(S);
        for (int t = 0; !s2.empty(); t++) {
            unordered_set<int> s3;
            for (auto & stop : s2) {
                auto rs = m.equal_range(stop);
                for (auto it = rs.first; it != rs.second; it++) {
                    int r = it->second;
                    if (v[r])
                        continue;
                    for (int & s : routes[r]) {
                        if (s == T)
                            return t + 1;
                        if (s1.find(s) == s1.end())
                            s3.insert(s);
                        s1.insert(s);
                    }
                    v[r] = true;
                }
            }
            s2.swap(s3);
        }
        return -1;
    }
};