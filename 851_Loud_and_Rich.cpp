class Solution {
    unordered_map<int, vector<int>> m;
    vector<int> v;

    int f(int a, vector<int>& q) {
        if (v[a] >= 0) return v[a];
        v[a] = a;
        for (int & i : m[a]) {
            if (q[f(i, q)] < q[v[a]])
                v[a] = f(i, q);
        }
        return v[a];
    }
public:
    vector<int> loudAndRich(vector<vector<int>>& richer, vector<int>& quiet) {
        int n = quiet.size();
        v = vector<int>(n, -1);
        for (auto & p : richer)
            m[p[1]].push_back(p[0]);
        for (int i = 0; i < n; i++) {
            v[i] = f(i, quiet);
        }
        return v;
    }
};