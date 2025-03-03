class Solution {
public:
    void gameOfLife(vector<vector<int>>& board) {
        int m = board.size(), n = m ? board[0].size() : 0;
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                int c = 0;
                for (int i2 = max(i - 1, 0); i2 < min(i + 2, m); i2++)
                    for (int j2 = max(j - 1, 0); j2 < min(j + 2, n); j2++)
                        if (board[i2][j2] & 1)
                            c++;
                if (c == 3 || c - board[i][j] == 3)
                    board[i][j] |= 2;
            }
        }
        for (int i = 0; i < m; i++)
            for (int j = 0; j < n; j++)
                board[i][j] >>= 1;
    }
};