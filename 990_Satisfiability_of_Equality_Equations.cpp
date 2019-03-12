class Solution {
    void f(vector<int>& v, vector<vector<int>>& adj, int i, int color) {
        v[i] = color;
        for (int i2 : adj[i])
            if (v[i2] == -1)
                f(v, adj, i2, color);
    }
public:
    bool equationsPossible(vector<string>& equations) {
        vector<int> v(26, -1);
        vector<vector<int>> adj(26);
        for (string & e : equations) if (e[1] == '=') {
            adj[e[0] - 'a'].push_back(e[3] - 'a');
            adj[e[3] - 'a'].push_back(e[0] - 'a');
        }
        for (int color = 1, c = 0; c < 26; c++) if (v[c] == -1) {
            f(v, adj, c, color);
            color++;
        }
        for (string & e : equations) if (e[1] == '!') {
            if (v[e[0] - 'a'] == v[e[3] - 'a'])
                return false;
        }
        return true;
    }
};