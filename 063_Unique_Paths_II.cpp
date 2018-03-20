class Solution {
public:
    int uniquePathsWithObstacles(vector<vector<int>>& obstacleGrid) {
        vector<int> v(obstacleGrid[0].size(), 0);
        v[0] = 1;
        for (int i = 0; i < obstacleGrid.size(); ++i) {
            for (int j = 0; j < obstacleGrid[0].size(); ++j) {
                if (j > 0) {
                    v[j] += v[j-1];
                }
                if (obstacleGrid[i][j] == 1) {
                    v[j] = 0;
                }
            }
        }
        return v[obstacleGrid[0].size()-1];
    }
};