class Solution {
    bool can(vector<vector<char>>& board, int r, int c, char v) {
        int r2 = r / 3 * 3, c2 = c / 3 * 3;
        for (int i = 0; i < 9; i++) {
            if (board[r][i] == v || board[i][c] == v) return false;
            if (board[r2 + i / 3][c2 + i % 3] == v) return false;
        }
        return true;
    }
    bool solve(vector<vector<char>>& board) {
        for (int i = 0; i < 9; i++) {
            for (int j = 0; j < 9; j++) {
                if (board[i][j] != '.') continue;
                for (char v = '1'; v <= '9'; v++) {
                    if (!can(board, i, j, v))
                        continue;
                    board[i][j] = v;
                    if (solve(board))
                        return true;
                    board[i][j] = '.';
                }
                return false;
            }
        }
        return true;
    }
public:
    void solveSudoku(vector<vector<char>>& board) {
        solve(board);
    }
};