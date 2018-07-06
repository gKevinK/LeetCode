class Solution {
public:
    int findCircleNum(vector<vector<int>>& M) {
        int n = M.size(), r = 0;
        vector<int> v(n, 0);
        for (int i = 0; i < n; i++) {
            if (v[i] != 0) continue;
            r++;
            queue<int> q;
            v[i] = r;
            q.push(i);
            while (!q.empty()) {
                int t = q.front();
                q.pop();
                for (int t2 = 0; t2 < n; t2++) {
                    if (t2 == t || M[t][t2] == 0 || v[t2] != 0) continue;
                    q.push(t2);
                    v[t2] = r;
                }
            }
        }
        return r;
    }
};