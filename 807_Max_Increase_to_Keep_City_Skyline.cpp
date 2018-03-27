class Solution {
public:
    int maxIncreaseKeepingSkyline(vector<vector<int>>& grid) {
        if (grid.size() == 0 || grid[0].size() == 0)
            return 0;
        int row = grid.size(), col = grid[0].size();
        vector<int> mr(row, 0), mc(col, 0);
        for (int i = 0; i < row; i++) {
            for (int j = 0; j < col; j++) {
                mr[i] = max(mr[i], grid[i][j]);
                mc[j] = max(mc[j], grid[i][j]);
            }
        }
        int sum = 0;
        for (int i = 0; i < row; i++) {
            for (int j = 0; j < col; j++) {
                sum += min(mr[i], mc[j]) - grid[i][j];
            }
        }
        return sum;
    }
};