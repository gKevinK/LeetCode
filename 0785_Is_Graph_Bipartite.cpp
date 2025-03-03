class Solution {
public:
    bool isBipartite(vector<vector<int>>& graph) {
        unordered_set<int> s1, s2;
        for (int i = 0; i < graph.size(); i++) {
            if (s1.find(i) != s1.end() || s2.find(i) != s2.end()) continue;
            s1.insert(i);
            vector<int> v1 = { i }, v2;
            while (!v1.empty()) {
                for (int i1 : v1) {
                    for (int ii : graph[i1]) {
                        if (s1.find(ii) != s1.end()) return false;
                        if (s2.find(ii) != s2.end()) continue;
                        v2.push_back(ii);
                        s2.insert(ii);
                    }
                }
                v1.clear();
                v1.swap(v2);
                s1.swap(s2);
            }
        }
        return true;
    }
};