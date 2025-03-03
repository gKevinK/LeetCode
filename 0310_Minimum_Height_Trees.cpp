class Solution {
public:
    vector<int> findMinHeightTrees(int n, vector<pair<int, int>>& edges) {
        vector<unordered_set<int>> neighbor(n, unordered_set<int>());
        for (auto & p : edges) {
            neighbor[p.first].insert(p.second);
            neighbor[p.second].insert(p.first);
        }
        vector<int> b1, b2;
        int count = 0;
        for (int i = 0; i < neighbor.size(); i++)
            if (neighbor[i].size() <= 1)
                b1.push_back(i);
        while (!b1.empty()) {
            if (n - count == b1.size()) return b1;
            for (int & node : b1) {
                int root = *neighbor[node].begin();
                neighbor[root].erase(node);
                if (neighbor[root].size() == 1)
                    b2.push_back(root);
            }
            count += b1.size();
            b1.clear();
            b1.swap(b2);
        }
        return {};
    }
};