class Solution {
    void f(vector<vector<char>>& grid, int i, int j) {
        queue<pair<int, int>> q;
        q.push(make_pair(i, j));
        while (!q.empty()) {
            int i1 = q.front().first, j1 = q.front().second;
            q.pop();
            grid[i1][j1] = '0';
            if (i1+1 < grid.size() && grid[i1+1][j1] == '1') {
                q.push(make_pair(i1+1, j1));
                grid[i1+1][j1] = '0';
            }
            if (i1-1 >= 0 && grid[i1-1][j1] == '1') {
                q.push(make_pair(i1-1, j1));
                grid[i1-1][j1] = '0';
            }
            if (j1+1 < grid[0].size() && grid[i1][j1+1] == '1') {
                q.push(make_pair(i1, j1+1));
                grid[i1][j1+1] = '0';
            }
            if (j1-1 >= 0 && grid[i1][j1-1] == '1') {
                q.push(make_pair(i1, j1-1));
                grid[i1][j1-1] = '0';
            }
        }
    }
public:
    int numIslands(vector<vector<char>>& grid) {
        if (grid.size() == 0) return 0;
        int num = 0;
        for (int i = 0; i < grid.size(); i++) {
            for (int j = 0; j < grid[0].size(); j++) {
                if (grid[i][j] == '1') {
                    f(grid, i, j);
                    num++;
                }
            }
        }
        return num;
    }
};