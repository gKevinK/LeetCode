class Solution {
public:
    bool isValidSudoku(vector<vector<char>>& board) {
        bool a1[9][9] = { false }, a2[9][9] = { false }, a3[9][9] = { false };
        for (int i = 0; i < 9; i++)
            for (int j = 0; j < 9; j++)
                if (board[i][j] != '.') {
                    int n = board[i][j] - '0' - 1, k = i / 3 + j / 3 * 3;
                    if (a1[i][n] || a2[j][n] || a3[k][n]) return false;
                    a1[i][n] = a2[j][n] = a3[k][n] = true;
                }
        return true;
    }
};