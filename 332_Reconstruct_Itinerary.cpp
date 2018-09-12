class Solution {
    void f(unordered_map<string, multiset<string>>& m, vector<string>& p, string airport) {
        while (!m[airport].empty()) {
            string next = *m[airport].begin();
            m[airport].erase(m[airport].begin());
            f(m, p, next);
        }
        p.push_back(airport);
    }
public:
    vector<string> findItinerary(vector<pair<string, string>> tickets) {
        unordered_map<string, multiset<string>> m;
        for (auto & t : tickets)
            m[t.first].insert(t.second);
        vector<string> p;
        f(m, p, "JFK");
        return vector<string>(p.rbegin(), p.rend());
    }
};