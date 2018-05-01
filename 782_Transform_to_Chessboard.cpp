class Solution {
public:
    int movesToChessboard(vector<vector<int>>& board) {
        int m = board.size(), n = board[0].size();
        for (int i = 1; i < m; i++)
            for (int j = 1; j < n; j++)
                if ((board[i][j] + board[0][j] + board[i][0] + board[0][0]) % 2)
                    return -1;
        vector<vector<int>> v1(2, vector<int>(2, 0)), v2(2, vector<int>(2, 0));
        for (int i = 0; i < m; i++)
            v1[i % 2][board[i][0]]++;
        for (int j = 0; j < n; j++)
            v2[j % 2][board[0][j]]++;
        if (v1[0][0] != v1[1][1])
            v1[0][0] = INT_MAX;
        if (v1[0][1] != v1[1][0])
            v1[0][1] = INT_MAX;
        if (v2[0][0] != v2[1][1])
            v2[0][0] = INT_MAX;
        if (v2[0][1] != v2[1][0])
            v2[0][1] = INT_MAX;
        int s1 = min(v1[0][0], v1[0][1]), s2 = min(v2[0][0], v2[0][1]);
        return (s1 != INT_MAX && s2 != INT_MAX) ? s1 + s2 : -1;
    }
};