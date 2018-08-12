class Solution {
public:
    int calculateMinimumHP(vector<vector<int>>& dungeon) {
        int m = dungeon.size();
        int n = dungeon[0].size();
        int map[m][n];
        map[m-1][n-1] = max(1, 1 - dungeon[m-1][n-1]);
        for (int j = n - 2; j >= 0; --j) {
            map[m-1][j] = max(1, - dungeon[m-1][j] + map[m-1][j+1]);
        }
        for (int i = m - 2; i >= 0; --i) {
            map[i][n-1] = max(1, - dungeon[i][n-1] + map[i+1][n-1]);
        }
        for (int i = m - 2; i >= 0; --i) {
            for (int j = n - 2; j >= 0; --j) {
                map[i][j] = max(1, - dungeon[i][j] + min(map[i][j+1], map[i+1][j]));
            }
        }
        return map[0][0];
    }
};