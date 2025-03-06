class Solution {
    void f(vector<vector<int>>& graph, vector<int>& path, vector<vector<int>>& res) {
        if (path.back() == graph.size() - 1) {
            res.push_back(path);
            return;
        }
        for (int& i : graph[path.back()]) {
            path.push_back(i);
            f(graph, path, res);
            path.pop_back();
        }
    }
public:
    vector<vector<int>> allPathsSourceTarget(vector<vector<int>>& graph) {
        vector<vector<int>> res;
        vector<int> path;
        path.push_back(0);
        f(graph, path, res);
        return res;
    }
};