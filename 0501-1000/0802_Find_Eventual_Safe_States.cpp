class Solution {
public:
    vector<int> eventualSafeNodes(vector<vector<int>>& graph) {
        vector<vector<int>> g2(graph.size(), vector<int>());
        vector<int> v, r;
        for (int i = 0; i < graph.size(); i++) {
            for (int & t : graph[i]) {
                g2[t].push_back(i);
            }
            v.push_back(graph[i].size());
        }
        for (int i = 0; i < graph.size(); i++) {
            if (v[i] == 0) {
                r.push_back(i);
            }
        }
        for (int t = 0; t < r.size(); t++) {
            for (const int & j : g2[r[t]]) {
                v[j]--;
                if (v[j] == 0)
                    r.push_back(j);
            }
        }
        sort(r.begin(), r.end());
        return r;
    }
};