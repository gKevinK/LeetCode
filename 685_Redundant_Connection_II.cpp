class Solution {
public:
    vector<int> findRedundantDirectedConnection(vector<vector<int>>& edges) {
        int n = edges.size(), tf = 0, sp = 0;
        vector<int> parent(n + 1, 0);
        for (auto & e : edges) {
            if (parent[e[1]]) {
                tf = e[1];
                sp = e[0];
            } else
                parent[e[1]] = e[0];
        }
        if (tf) {
            for (int i = parent[tf]; i != 0; i = parent[i])
                if (i == tf)
                    return { parent[tf], tf };
            return { sp, tf };
        } else {
            int i = parent[1], j = 1;
            while (i != j) {
                i = parent[parent[i]];
                j = parent[j];
            }
            while (parent[i] != INT_MAX) {
                j = parent[i];
                parent[i] = INT_MAX;
                i = j;
            }
            for (int i = n - 1; i >= 0; --i) {
                if (parent[edges[i][0]] == INT_MAX && parent[edges[i][1]] == INT_MAX)
                    return edges[i];
            }
        }
        return {};
    }
};