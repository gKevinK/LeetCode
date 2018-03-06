class Solution {
    bool check(vector<string>& b, char c) {
        for (int i = 0; i < 3; i++) {
            if (b[i][0] == c && b[i][1] == c && b[i][2] == c) {
                return true;
            }
        }
        for (int j = 0; j < 3; j++) {
            if (b[0][j] == c && b[1][j] == c && b[2][j] == c) {
                return true;
            }
        }
        if ((b[0][0] == c && b[1][1] == c && b[2][2] == c) || (b[2][0] == c && b[1][1] == c && b[0][2] == c))
            return true;
        return false;
    }
public:
    bool validTicTacToe(vector<string>& board) {
        int x = 0, y = 0;
        for (auto & i : board) {
            for (char & c : i) {
                if (c == 'X') x++;
                else if (c == 'O') y++;
            }
        }
        if (x > y + 1 || x < y) return false;
        if (check(board, 'X') && check(board, 'O')) return false;
        if (check(board, 'X') && x == y) return false;
        if (check(board, 'O') && x == y + 1) return false;
        return true;
    }
};