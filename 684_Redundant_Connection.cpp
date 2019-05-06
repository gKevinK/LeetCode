class Solution {
public:
    vector<int> findRedundantConnection(vector<vector<int>>& edges) {
        int n = edges.size();
        vector<int> v(n + 1, 0);
        for (auto & e : edges) {
            int left = e[0], right = e[1];
            while (v[left]) left = v[left];
            while (v[right]) right = v[right];
            if (left != 0 && left == right) return e;
            v[right] = left;
        }
        return {};
    }
};